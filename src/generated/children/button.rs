// 🤖 This file is generated!

use crate::*;
/// The `<button>` element's children.
#[derive(Clone)]
pub enum ButtonChild {
    Abbreviation(Abbreviation),
    Anchor(Anchor),
    Audio(Audio),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Cite(Cite),
    Code(Code),
    Custom(Custom),
    Data(Data),
    DataList(DataList),
    Definition(Definition),
    Deleted(Deleted),
    Embed(Embed),
    Emphasis(Emphasis),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    Map(Map),
    MapArea(MapArea),
    Mark(Mark),
    Metadata(Metadata),
    Meter(Meter),
    NoScript(NoScript),
    Object(Object),
    Output(Output),
    Picture(Picture),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    Sample(Sample),
    Script(Script),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Underline(Underline),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for ButtonChild {
    fn from(child: Abbreviation) -> Self {
        ButtonChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for ButtonChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        ButtonChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for ButtonChild {
    fn from(child: Anchor) -> Self {
        ButtonChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for ButtonChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        ButtonChild::Anchor(builder.build())
    }
}
impl From<Audio> for ButtonChild {
    fn from(child: Audio) -> Self {
        ButtonChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for ButtonChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        ButtonChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for ButtonChild {
    fn from(child: BidirectionalIsolate) -> Self {
        ButtonChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for ButtonChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        ButtonChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for ButtonChild {
    fn from(child: BidirectionalOverride) -> Self {
        ButtonChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for ButtonChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        ButtonChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for ButtonChild {
    fn from(child: Bold) -> Self {
        ButtonChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for ButtonChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        ButtonChild::Bold(builder.build())
    }
}
impl From<Button> for ButtonChild {
    fn from(child: Button) -> Self {
        ButtonChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for ButtonChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        ButtonChild::Button(builder.build())
    }
}
impl From<Canvas> for ButtonChild {
    fn from(child: Canvas) -> Self {
        ButtonChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for ButtonChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        ButtonChild::Canvas(builder.build())
    }
}
impl From<Cite> for ButtonChild {
    fn from(child: Cite) -> Self {
        ButtonChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for ButtonChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        ButtonChild::Cite(builder.build())
    }
}
impl From<Code> for ButtonChild {
    fn from(child: Code) -> Self {
        ButtonChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for ButtonChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        ButtonChild::Code(builder.build())
    }
}
impl From<Custom> for ButtonChild {
    fn from(child: Custom) -> Self {
        ButtonChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for ButtonChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        ButtonChild::Custom(builder.build())
    }
}
impl From<Data> for ButtonChild {
    fn from(child: Data) -> Self {
        ButtonChild::Data(child)
    }
}
impl From<builders::DataBuilder> for ButtonChild {
    fn from(builder: builders::DataBuilder) -> Self {
        ButtonChild::Data(builder.build())
    }
}
impl From<DataList> for ButtonChild {
    fn from(child: DataList) -> Self {
        ButtonChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for ButtonChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        ButtonChild::DataList(builder.build())
    }
}
impl From<Definition> for ButtonChild {
    fn from(child: Definition) -> Self {
        ButtonChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for ButtonChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        ButtonChild::Definition(builder.build())
    }
}
impl From<Deleted> for ButtonChild {
    fn from(child: Deleted) -> Self {
        ButtonChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for ButtonChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        ButtonChild::Deleted(builder.build())
    }
}
impl From<Embed> for ButtonChild {
    fn from(child: Embed) -> Self {
        ButtonChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for ButtonChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        ButtonChild::Embed(builder.build())
    }
}
impl From<Emphasis> for ButtonChild {
    fn from(child: Emphasis) -> Self {
        ButtonChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for ButtonChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        ButtonChild::Emphasis(builder.build())
    }
}
impl From<Image> for ButtonChild {
    fn from(child: Image) -> Self {
        ButtonChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for ButtonChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        ButtonChild::Image(builder.build())
    }
}
impl From<InlineFrame> for ButtonChild {
    fn from(child: InlineFrame) -> Self {
        ButtonChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for ButtonChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        ButtonChild::InlineFrame(builder.build())
    }
}
impl From<Input> for ButtonChild {
    fn from(child: Input) -> Self {
        ButtonChild::Input(child)
    }
}
impl From<builders::InputBuilder> for ButtonChild {
    fn from(builder: builders::InputBuilder) -> Self {
        ButtonChild::Input(builder.build())
    }
}
impl From<Inserted> for ButtonChild {
    fn from(child: Inserted) -> Self {
        ButtonChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for ButtonChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        ButtonChild::Inserted(builder.build())
    }
}
impl From<Italic> for ButtonChild {
    fn from(child: Italic) -> Self {
        ButtonChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for ButtonChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        ButtonChild::Italic(builder.build())
    }
}
impl From<Keyboard> for ButtonChild {
    fn from(child: Keyboard) -> Self {
        ButtonChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for ButtonChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        ButtonChild::Keyboard(builder.build())
    }
}
impl From<Label> for ButtonChild {
    fn from(child: Label) -> Self {
        ButtonChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for ButtonChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        ButtonChild::Label(builder.build())
    }
}
impl From<LineBreak> for ButtonChild {
    fn from(child: LineBreak) -> Self {
        ButtonChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for ButtonChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        ButtonChild::LineBreak(builder.build())
    }
}
impl From<Link> for ButtonChild {
    fn from(child: Link) -> Self {
        ButtonChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for ButtonChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        ButtonChild::Link(builder.build())
    }
}
impl From<Map> for ButtonChild {
    fn from(child: Map) -> Self {
        ButtonChild::Map(child)
    }
}
impl From<builders::MapBuilder> for ButtonChild {
    fn from(builder: builders::MapBuilder) -> Self {
        ButtonChild::Map(builder.build())
    }
}
impl From<MapArea> for ButtonChild {
    fn from(child: MapArea) -> Self {
        ButtonChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for ButtonChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        ButtonChild::MapArea(builder.build())
    }
}
impl From<Mark> for ButtonChild {
    fn from(child: Mark) -> Self {
        ButtonChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for ButtonChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        ButtonChild::Mark(builder.build())
    }
}
impl From<Metadata> for ButtonChild {
    fn from(child: Metadata) -> Self {
        ButtonChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for ButtonChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        ButtonChild::Metadata(builder.build())
    }
}
impl From<Meter> for ButtonChild {
    fn from(child: Meter) -> Self {
        ButtonChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for ButtonChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        ButtonChild::Meter(builder.build())
    }
}
impl From<NoScript> for ButtonChild {
    fn from(child: NoScript) -> Self {
        ButtonChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for ButtonChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        ButtonChild::NoScript(builder.build())
    }
}
impl From<Object> for ButtonChild {
    fn from(child: Object) -> Self {
        ButtonChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for ButtonChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        ButtonChild::Object(builder.build())
    }
}
impl From<Output> for ButtonChild {
    fn from(child: Output) -> Self {
        ButtonChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for ButtonChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        ButtonChild::Output(builder.build())
    }
}
impl From<Picture> for ButtonChild {
    fn from(child: Picture) -> Self {
        ButtonChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for ButtonChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        ButtonChild::Picture(builder.build())
    }
}
impl From<Progress> for ButtonChild {
    fn from(child: Progress) -> Self {
        ButtonChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for ButtonChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        ButtonChild::Progress(builder.build())
    }
}
impl From<Quote> for ButtonChild {
    fn from(child: Quote) -> Self {
        ButtonChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for ButtonChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        ButtonChild::Quote(builder.build())
    }
}
impl From<Ruby> for ButtonChild {
    fn from(child: Ruby) -> Self {
        ButtonChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for ButtonChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        ButtonChild::Ruby(builder.build())
    }
}
impl From<Sample> for ButtonChild {
    fn from(child: Sample) -> Self {
        ButtonChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for ButtonChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        ButtonChild::Sample(builder.build())
    }
}
impl From<Script> for ButtonChild {
    fn from(child: Script) -> Self {
        ButtonChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for ButtonChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        ButtonChild::Script(builder.build())
    }
}
impl From<Select> for ButtonChild {
    fn from(child: Select) -> Self {
        ButtonChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for ButtonChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        ButtonChild::Select(builder.build())
    }
}
impl From<Slot> for ButtonChild {
    fn from(child: Slot) -> Self {
        ButtonChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for ButtonChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        ButtonChild::Slot(builder.build())
    }
}
impl From<Small> for ButtonChild {
    fn from(child: Small) -> Self {
        ButtonChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for ButtonChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        ButtonChild::Small(builder.build())
    }
}
impl From<Span> for ButtonChild {
    fn from(child: Span) -> Self {
        ButtonChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for ButtonChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        ButtonChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for ButtonChild {
    fn from(child: StrikeThrough) -> Self {
        ButtonChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for ButtonChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        ButtonChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for ButtonChild {
    fn from(child: Strong) -> Self {
        ButtonChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for ButtonChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        ButtonChild::Strong(builder.build())
    }
}
impl From<SubScript> for ButtonChild {
    fn from(child: SubScript) -> Self {
        ButtonChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for ButtonChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        ButtonChild::SubScript(builder.build())
    }
}
impl From<SupScript> for ButtonChild {
    fn from(child: SupScript) -> Self {
        ButtonChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for ButtonChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        ButtonChild::SupScript(builder.build())
    }
}
impl From<Template> for ButtonChild {
    fn from(child: Template) -> Self {
        ButtonChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for ButtonChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        ButtonChild::Template(builder.build())
    }
}
impl From<TextArea> for ButtonChild {
    fn from(child: TextArea) -> Self {
        ButtonChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for ButtonChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        ButtonChild::TextArea(builder.build())
    }
}
impl From<Time> for ButtonChild {
    fn from(child: Time) -> Self {
        ButtonChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for ButtonChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        ButtonChild::Time(builder.build())
    }
}
impl From<Underline> for ButtonChild {
    fn from(child: Underline) -> Self {
        ButtonChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for ButtonChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        ButtonChild::Underline(builder.build())
    }
}
impl From<Variable> for ButtonChild {
    fn from(child: Variable) -> Self {
        ButtonChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for ButtonChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        ButtonChild::Variable(builder.build())
    }
}
impl From<Video> for ButtonChild {
    fn from(child: Video) -> Self {
        ButtonChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for ButtonChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        ButtonChild::Video(builder.build())
    }
}
impl From<WordBreak> for ButtonChild {
    fn from(child: WordBreak) -> Self {
        ButtonChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for ButtonChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        ButtonChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for ButtonChild {
    fn from(s: &'static str) -> Self {
        ButtonChild::Text(s.into())
    }
}
impl From<String> for ButtonChild {
    fn from(s: String) -> Self {
        ButtonChild::Text(s.into())
    }
}
impl From<CowStr> for ButtonChild {
    fn from(s: CowStr) -> Self {
        ButtonChild::Text(s)
    }
}
impl std::fmt::Debug for ButtonChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Button(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Code(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Data(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Image(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Input(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Label(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Link(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Map(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Object(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Output(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Script(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Select(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Small(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Span(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Template(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Time(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Video(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            ButtonChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for ButtonChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Audio(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Bold(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Button(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Cite(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Code(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Custom(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Data(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::DataList(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Definition(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Embed(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Image(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Input(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Italic(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Label(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Link(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Map(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Mark(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Meter(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Object(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Output(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Picture(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Progress(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Quote(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Sample(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Script(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Select(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Slot(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Small(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Span(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Strong(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Template(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Time(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Underline(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Variable(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Video(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            ButtonChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
