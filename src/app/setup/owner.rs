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

#[cfg(test)]
mod tests {
    use super::*;
    use topcoat::view::ViewExt;

    async fn render_field(field: OwnerFormValidation, ty: InputType) -> String {
        let cx = &Cx::new();
        view! { input(ty: ty, inner_attrs: field.attrs(cx)) }
            .single()
            .await
            .unwrap()
            .render(cx)
    }

    #[tokio::test]
    async fn username_attrs_drive_validator_messages() {
        let html = render_field(OwnerFormValidation::Username, InputType::Text).await;
        assert!(html.contains("name=\"username\""), "{html}");
        assert!(html.contains("minlength=\"3\""), "{html}");
        assert!(html.contains("maxlength=\"30\""), "{html}");
        assert!(html.contains("Must be at least 3 characters."), "{html}");
        assert!(html.contains("Must be at most 30 characters."), "{html}");
        assert!(html.contains("data-topcoat-bind:hidden"), "{html}");
        assert!(html.contains("validator"), "{html}");
    }

    #[tokio::test]
    async fn password_attrs_drive_validator_messages() {
        let html = render_field(OwnerFormValidation::Password, InputType::Password).await;
        assert!(html.contains("name=\"password\""), "{html}");
        assert!(html.contains("type=\"password\""), "{html}");
        assert!(html.contains("minlength=\"3\""), "{html}");
        assert!(html.contains("Must be at least 3 characters."), "{html}");
        assert!(!html.contains("maxlength"), "{html}");
        assert!(!html.contains("Must be at most"), "{html}");
    }

    #[tokio::test]
    async fn required_fields_show_required_message() {
        for (field, ty) in [
            (OwnerFormValidation::Username, InputType::Text),
            (OwnerFormValidation::Password, InputType::Password),
        ] {
            let html = render_field(field, ty).await;
            assert!(html.contains("required"), "{html}");
            assert!(html.contains("This field is required."), "{html}");
        }
    }
}
