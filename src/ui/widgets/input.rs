use crate::{
    math::{vec2, Vec2},
    ui::{widgets::Editbox, ElementState, Id, Layout, Ui, UiContent},
};

pub struct InputText<'a> {
    id: Id,
    label: &'a str,
    size: Option<Vec2>,
    password: bool,
    numbers: bool,
    ratio: f32,
    pos: Option<Vec2>,
    margin: Option<Vec2>,
    input_font_size: Option<f32>,
    label_font_size: Option<f32>,
}

impl<'a> InputText<'a> {
    pub const fn new(id: Id) -> InputText<'a> {
        InputText {
            id,
            size: None,
            label: "",
            numbers: false,
            password: false,
            ratio: 1.0,
            pos: None,
            margin: None,
            input_font_size: None,
            label_font_size: None,
        }
    }

    pub const fn label<'b>(self, label: &'b str) -> InputText<'b> {
        InputText { label, ..self }
    }

    pub const fn size(self, size: Vec2) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    pub const fn position(self, pos: Vec2) -> Self {
        Self {
            pos: Some(pos),
            ..self
        }
    }

    pub const fn password(self, password: bool) -> Self {
        Self { password, ..self }
    }

    pub const fn ratio(self, ratio: f32) -> Self {
        Self { ratio, ..self }
    }

    pub const fn filter_numbers(self) -> Self {
        Self {
            numbers: true,
            ..self
        }
    }

    pub const fn margin(self, margin: Vec2) -> Self {
        Self {
            margin: Some(margin),
            ..self
        }
    }

    pub const fn input_font_size(self, font_size: f32) -> Self {
        Self {
            input_font_size: Some(font_size),
            ..self
        }
    }

    pub const fn label_font_size(self, font_size: f32) -> Self {
        Self {
            label_font_size: Some(font_size),
            ..self
        }
    }

    pub fn ui(self, ui: &mut Ui, data: &mut String) {
        let context = ui.get_active_window_context();

        let label_size = context.window.painter.content_with_margins_size(
            &context.style.editbox_style,
            &UiContent::Label((&*data).into()),
        );

        let size = self.size.unwrap_or(vec2(
            context.window.cursor.area.w - context.style.margin * 2. - context.window.cursor.ident,
            label_size.y.max(19.),
        ));

        let pos = self
            .pos
            .unwrap_or_else(|| context.window.cursor.fit(size, Layout::Vertical));

        let editbox_area_w = if self.label.is_empty() {
            size.x
        } else {
            size.x * self.ratio - 15.
        };
        let mut editbox = Editbox::new(self.id, Vec2::new(editbox_area_w, size.y))
            .password(self.password)
            .position(pos)
            .multiline(false);
        if let Some(font_size) = self.input_font_size {
            editbox = editbox.font_size(font_size);
        }

        if let Some(margin) = self.margin {
            editbox = editbox.margin(margin);
        }

        if self.numbers {
            editbox = editbox.filter(&|character| {
                character.is_digit(10) || character == '.' || character == '-'
            });
        }
        editbox.ui(ui, data);

        let context = ui.get_active_window_context();

        let label_style = if let Some(font_size) = self.label_font_size {
            let mut style = context.style.label_style.clone();
            style.font_size = font_size as u16;
            style
        } else {
            context.style.label_style.clone()
        };
        if self.label.is_empty() == false {
            context.window.painter.draw_element_label(
                &label_style,
                Vec2::new(pos.x + size.x * self.ratio, pos.y),
                self.label,
                ElementState {
                    focused: context.focused,
                    ..Default::default()
                },
            );
        }
    }
}

impl Ui {
    pub fn input_text(&mut self, id: Id, label: &str, data: &mut String) {
        InputText::new(id).label(label).ui(self, data);
    }

    pub fn input_password(&mut self, id: Id, label: &str, data: &mut String) {
        InputText::new(id)
            .label(label)
            .password(true)
            .ui(self, data);
    }
}
