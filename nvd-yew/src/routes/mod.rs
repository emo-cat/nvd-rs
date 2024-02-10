use yew::prelude::*;
use yew_router::prelude::*;

// use nvd_cvss::Cvss;
use cpe::VendorProducts;
use cve::CVEDetails;
use cve_list::CveInfoList;
use exp::ExploitInfoList;
use home::Home;
use page_not_found::PageNotFound;

mod cve;
mod cve_list;
// mod cvss;
mod cpe;
mod exp;
mod home;
mod page_not_found;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
  #[at("/cve/:id")]
  Cve { id: String },
  #[at("/cve/")]
  CveList,
  #[at("/cpe/")]
  Cpe,
  #[at("/exp/")]
  Exp,
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
      Route::Exp => {
        html! { <ExploitInfoList /> }
      }
      Route::NotFound => {
        html! { <PageNotFound /> }
      }
    }
  }
}
