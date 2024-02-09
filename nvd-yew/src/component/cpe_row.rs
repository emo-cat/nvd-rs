use crate::routes::Route;

use yew::prelude::*;
use yew_router::prelude::*;
use nvd_model::product::ProductWithVendor;

// 供应商，产品回调
#[derive(PartialEq, Clone, Properties)]
pub struct CpeProps {
  pub props: ProductWithVendor,
}

pub struct CPERow;

impl Component for CPERow {
  type Message = ();
  type Properties = CpeProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let CpeProps { props, .. } = ctx.props().clone();
    let product = props.product.clone();
    let vendor = props.vendor.clone();
    let update = product.updated_at.to_string();
    let name = product.name;
    let vendor_name = vendor.name;
    let description = product.description.unwrap_or(String::from("N/A"));
    html! {
    <>
        <tr class="table-group-divider">
          <th scope="row">
          <Link<Route> classes={classes!("text-reset")} to={Route::Cpe{}}>
             <i class="ti ti-external-link"></i>
              {vendor_name.clone()}
          </Link<Route>>
          </th>
          <td>{name}</td>
          <td>
            {update}
          </td>
        </tr>
        <tr class="table-success">
          <th scope="row" colspan="7" class="table table-active text-truncate" style="max-width: 150px;">{description}</th>
        </tr>
    </>
    }
  }
}
