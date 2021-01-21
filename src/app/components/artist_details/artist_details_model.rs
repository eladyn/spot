use crate::app::models::*;
use crate::app::{ActionDispatcher, AppAction, AppModel, BrowserAction, ListStore, Worker};
use std::ops::Deref;
use std::rc::Rc;

use super::ArtistDetails;

pub struct ArtistDetailsFactory {
    app_model: Rc<AppModel>,
    dispatcher: Box<dyn ActionDispatcher>,
    worker: Worker,
}

impl ArtistDetailsFactory {
    pub fn new(
        app_model: Rc<AppModel>,
        dispatcher: Box<dyn ActionDispatcher>,
        worker: Worker,
    ) -> Self {
        Self {
            app_model,
            dispatcher,
            worker,
        }
    }

    pub fn make_artist_details(&self, id: String) -> ArtistDetails {
        let model = ArtistDetailsModel {
            app_model: Rc::clone(&self.app_model),
            dispatcher: self.dispatcher.box_clone(),
        };
        ArtistDetails::new(id, model, self.worker.clone())
    }
}

pub struct ArtistDetailsModel {
    app_model: Rc<AppModel>,
    dispatcher: Box<dyn ActionDispatcher>,
}

impl ArtistDetailsModel {
    pub fn get_artist_name(&self) -> Option<impl Deref<Target = String> + '_> {
        self.app_model
            .map_state_opt(|s| s.browser_state.artist_state()?.artist.as_ref())
    }

    pub fn get_list_store(&self) -> Option<impl Deref<Target = ListStore<AlbumModel>> + '_> {
        self.app_model
            .map_state_opt(|s| Some(&s.browser_state.artist_state()?.albums))
    }

    pub fn load_artist_details(&self, id: String) {
        let api = self.app_model.get_spotify();
        self.dispatcher.dispatch_async(Box::pin(async move {
            let artist = api.get_artist(&id[..]).await?;
            Some(BrowserAction::SetArtistDetails(artist).into())
        }));
    }

    pub fn open_album(&self, id: &str) {
        self.dispatcher
            .dispatch(AppAction::ViewAlbum(id.to_string()));
    }
}
