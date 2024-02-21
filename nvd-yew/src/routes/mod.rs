use yew::prelude::*;
use yew_router::prelude::*;

use cpe::VendorProducts;
use cve::CVEDetails;
use cve_list::CveInfoList;
use home::Home;
use kb::KnowledgeBaseInfoList;
use page_not_found::PageNotFound;

mod cve;
mod cve_list;
// mod cvss;
mod cpe;
mod home;
mod kb;
mod page_not_found;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
  #[at("/cve/:id")]
  Cve { id: String },
  #[at("/cve/")]
  CveList,
  #[at("/cpe/")]
  Cpe,
  #[at("/kb/")]
  Kb,
  #[at("/")]
  Home,
  #[not_found]
  #[at("/404")]
  NotFound,
}

impl Route {
  pub(crate) fn switch(routes: Route) -> Html {
    match routes {
      Route::Home => {
        html! { <Home /> }
      }
      Route::CveList => {
        html! {<CveInfoList/>}
      }
      Route::Cve { id } => {
        html! {<CVEDetails id={id}/ >}
      }
      Route::Cpe => {
        html! {<VendorProducts/>}
      }
      Route::Kb => {
        html! { <KnowledgeBaseInfoList /> }
      }
      Route::NotFound => {
        html! { <PageNotFound /> }
      }
    }
  }
}
