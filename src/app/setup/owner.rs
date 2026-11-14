use crate::components::prelude::*;
use garde::Validate;
use schemars::JsonSchema;
use server_z_macros::FormValidation;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

#[derive(JsonSchema, Validate, FormValidation)]
struct OwnerForm {
    #[garde(ascii, length(min = 3, max = 30))]
    #[schemars(length(min = 3, max = 30))]
    username: String,
    #[garde(length(min = 3))]
    #[schemars(length(min = 3))]
    password: String,
}

#[page]
pub async fn setup_owner(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <form>
            input(
                label: Some("Username".into()),
                label_for: Some(OwnerFormValidation::Username.name().into()),
                inner_attrs: OwnerFormValidation::Username.attrs(cx)
            )
            input(
                ty: InputType::Password,
                label: Some("Password".into()),
                label_for: Some(OwnerFormValidation::Password.name().into()),
                inner_attrs: OwnerFormValidation::Password.attrs(cx)
            )
            button(attrs: attributes!(type="submit"), "Submit")
        </form>
    })
}
