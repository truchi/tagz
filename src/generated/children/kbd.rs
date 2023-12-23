// 🤖 This file is generated!

use crate::*;
/// The `<kbd>` element's children.
#[derive(Clone)]
pub enum KeyboardChild {
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
impl From<Abbreviation> for KeyboardChild {
    fn from(child: Abbreviation) -> Self {
        KeyboardChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for KeyboardChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        KeyboardChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for KeyboardChild {
    fn from(child: Anchor) -> Self {
        KeyboardChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for KeyboardChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        KeyboardChild::Anchor(builder.build())
    }
}
impl From<Audio> for KeyboardChild {
    fn from(child: Audio) -> Self {
        KeyboardChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for KeyboardChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        KeyboardChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for KeyboardChild {
    fn from(child: BidirectionalIsolate) -> Self {
        KeyboardChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for KeyboardChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        KeyboardChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for KeyboardChild {
    fn from(child: BidirectionalOverride) -> Self {
        KeyboardChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for KeyboardChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        KeyboardChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for KeyboardChild {
    fn from(child: Bold) -> Self {
        KeyboardChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for KeyboardChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        KeyboardChild::Bold(builder.build())
    }
}
impl From<Button> for KeyboardChild {
    fn from(child: Button) -> Self {
        KeyboardChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for KeyboardChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        KeyboardChild::Button(builder.build())
    }
}
impl From<Canvas> for KeyboardChild {
    fn from(child: Canvas) -> Self {
        KeyboardChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for KeyboardChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        KeyboardChild::Canvas(builder.build())
    }
}
impl From<Cite> for KeyboardChild {
    fn from(child: Cite) -> Self {
        KeyboardChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for KeyboardChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        KeyboardChild::Cite(builder.build())
    }
}
impl From<Code> for KeyboardChild {
    fn from(child: Code) -> Self {
        KeyboardChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for KeyboardChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        KeyboardChild::Code(builder.build())
    }
}
impl From<Custom> for KeyboardChild {
    fn from(child: Custom) -> Self {
        KeyboardChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for KeyboardChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        KeyboardChild::Custom(builder.build())
    }
}
impl From<Data> for KeyboardChild {
    fn from(child: Data) -> Self {
        KeyboardChild::Data(child)
    }
}
impl From<builders::DataBuilder> for KeyboardChild {
    fn from(builder: builders::DataBuilder) -> Self {
        KeyboardChild::Data(builder.build())
    }
}
impl From<DataList> for KeyboardChild {
    fn from(child: DataList) -> Self {
        KeyboardChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for KeyboardChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        KeyboardChild::DataList(builder.build())
    }
}
impl From<Definition> for KeyboardChild {
    fn from(child: Definition) -> Self {
        KeyboardChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for KeyboardChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        KeyboardChild::Definition(builder.build())
    }
}
impl From<Deleted> for KeyboardChild {
    fn from(child: Deleted) -> Self {
        KeyboardChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for KeyboardChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        KeyboardChild::Deleted(builder.build())
    }
}
impl From<Embed> for KeyboardChild {
    fn from(child: Embed) -> Self {
        KeyboardChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for KeyboardChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        KeyboardChild::Embed(builder.build())
    }
}
impl From<Emphasis> for KeyboardChild {
    fn from(child: Emphasis) -> Self {
        KeyboardChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for KeyboardChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        KeyboardChild::Emphasis(builder.build())
    }
}
impl From<Image> for KeyboardChild {
    fn from(child: Image) -> Self {
        KeyboardChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for KeyboardChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        KeyboardChild::Image(builder.build())
    }
}
impl From<InlineFrame> for KeyboardChild {
    fn from(child: InlineFrame) -> Self {
        KeyboardChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for KeyboardChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        KeyboardChild::InlineFrame(builder.build())
    }
}
impl From<Input> for KeyboardChild {
    fn from(child: Input) -> Self {
        KeyboardChild::Input(child)
    }
}
impl From<builders::InputBuilder> for KeyboardChild {
    fn from(builder: builders::InputBuilder) -> Self {
        KeyboardChild::Input(builder.build())
    }
}
impl From<Inserted> for KeyboardChild {
    fn from(child: Inserted) -> Self {
        KeyboardChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for KeyboardChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        KeyboardChild::Inserted(builder.build())
    }
}
impl From<Italic> for KeyboardChild {
    fn from(child: Italic) -> Self {
        KeyboardChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for KeyboardChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        KeyboardChild::Italic(builder.build())
    }
}
impl From<Keyboard> for KeyboardChild {
    fn from(child: Keyboard) -> Self {
        KeyboardChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for KeyboardChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        KeyboardChild::Keyboard(builder.build())
    }
}
impl From<Label> for KeyboardChild {
    fn from(child: Label) -> Self {
        KeyboardChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for KeyboardChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        KeyboardChild::Label(builder.build())
    }
}
impl From<LineBreak> for KeyboardChild {
    fn from(child: LineBreak) -> Self {
        KeyboardChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for KeyboardChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        KeyboardChild::LineBreak(builder.build())
    }
}
impl From<Link> for KeyboardChild {
    fn from(child: Link) -> Self {
        KeyboardChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for KeyboardChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        KeyboardChild::Link(builder.build())
    }
}
impl From<Map> for KeyboardChild {
    fn from(child: Map) -> Self {
        KeyboardChild::Map(child)
    }
}
impl From<builders::MapBuilder> for KeyboardChild {
    fn from(builder: builders::MapBuilder) -> Self {
        KeyboardChild::Map(builder.build())
    }
}
impl From<MapArea> for KeyboardChild {
    fn from(child: MapArea) -> Self {
        KeyboardChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for KeyboardChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        KeyboardChild::MapArea(builder.build())
    }
}
impl From<Mark> for KeyboardChild {
    fn from(child: Mark) -> Self {
        KeyboardChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for KeyboardChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        KeyboardChild::Mark(builder.build())
    }
}
impl From<Metadata> for KeyboardChild {
    fn from(child: Metadata) -> Self {
        KeyboardChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for KeyboardChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        KeyboardChild::Metadata(builder.build())
    }
}
impl From<Meter> for KeyboardChild {
    fn from(child: Meter) -> Self {
        KeyboardChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for KeyboardChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        KeyboardChild::Meter(builder.build())
    }
}
impl From<NoScript> for KeyboardChild {
    fn from(child: NoScript) -> Self {
        KeyboardChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for KeyboardChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        KeyboardChild::NoScript(builder.build())
    }
}
impl From<Object> for KeyboardChild {
    fn from(child: Object) -> Self {
        KeyboardChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for KeyboardChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        KeyboardChild::Object(builder.build())
    }
}
impl From<Output> for KeyboardChild {
    fn from(child: Output) -> Self {
        KeyboardChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for KeyboardChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        KeyboardChild::Output(builder.build())
    }
}
impl From<Picture> for KeyboardChild {
    fn from(child: Picture) -> Self {
        KeyboardChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for KeyboardChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        KeyboardChild::Picture(builder.build())
    }
}
impl From<Progress> for KeyboardChild {
    fn from(child: Progress) -> Self {
        KeyboardChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for KeyboardChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        KeyboardChild::Progress(builder.build())
    }
}
impl From<Quote> for KeyboardChild {
    fn from(child: Quote) -> Self {
        KeyboardChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for KeyboardChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        KeyboardChild::Quote(builder.build())
    }
}
impl From<Ruby> for KeyboardChild {
    fn from(child: Ruby) -> Self {
        KeyboardChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for KeyboardChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        KeyboardChild::Ruby(builder.build())
    }
}
impl From<Sample> for KeyboardChild {
    fn from(child: Sample) -> Self {
        KeyboardChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for KeyboardChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        KeyboardChild::Sample(builder.build())
    }
}
impl From<Script> for KeyboardChild {
    fn from(child: Script) -> Self {
        KeyboardChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for KeyboardChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        KeyboardChild::Script(builder.build())
    }
}
impl From<Select> for KeyboardChild {
    fn from(child: Select) -> Self {
        KeyboardChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for KeyboardChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        KeyboardChild::Select(builder.build())
    }
}
impl From<Slot> for KeyboardChild {
    fn from(child: Slot) -> Self {
        KeyboardChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for KeyboardChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        KeyboardChild::Slot(builder.build())
    }
}
impl From<Small> for KeyboardChild {
    fn from(child: Small) -> Self {
        KeyboardChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for KeyboardChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        KeyboardChild::Small(builder.build())
    }
}
impl From<Span> for KeyboardChild {
    fn from(child: Span) -> Self {
        KeyboardChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for KeyboardChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        KeyboardChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for KeyboardChild {
    fn from(child: StrikeThrough) -> Self {
        KeyboardChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for KeyboardChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        KeyboardChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for KeyboardChild {
    fn from(child: Strong) -> Self {
        KeyboardChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for KeyboardChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        KeyboardChild::Strong(builder.build())
    }
}
impl From<SubScript> for KeyboardChild {
    fn from(child: SubScript) -> Self {
        KeyboardChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for KeyboardChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        KeyboardChild::SubScript(builder.build())
    }
}
impl From<SupScript> for KeyboardChild {
    fn from(child: SupScript) -> Self {
        KeyboardChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for KeyboardChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        KeyboardChild::SupScript(builder.build())
    }
}
impl From<Template> for KeyboardChild {
    fn from(child: Template) -> Self {
        KeyboardChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for KeyboardChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        KeyboardChild::Template(builder.build())
    }
}
impl From<TextArea> for KeyboardChild {
    fn from(child: TextArea) -> Self {
        KeyboardChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for KeyboardChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        KeyboardChild::TextArea(builder.build())
    }
}
impl From<Time> for KeyboardChild {
    fn from(child: Time) -> Self {
        KeyboardChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for KeyboardChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        KeyboardChild::Time(builder.build())
    }
}
impl From<Underline> for KeyboardChild {
    fn from(child: Underline) -> Self {
        KeyboardChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for KeyboardChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        KeyboardChild::Underline(builder.build())
    }
}
impl From<Variable> for KeyboardChild {
    fn from(child: Variable) -> Self {
        KeyboardChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for KeyboardChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        KeyboardChild::Variable(builder.build())
    }
}
impl From<Video> for KeyboardChild {
    fn from(child: Video) -> Self {
        KeyboardChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for KeyboardChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        KeyboardChild::Video(builder.build())
    }
}
impl From<WordBreak> for KeyboardChild {
    fn from(child: WordBreak) -> Self {
        KeyboardChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for KeyboardChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        KeyboardChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for KeyboardChild {
    fn from(s: &'static str) -> Self {
        KeyboardChild::Text(s.into())
    }
}
impl From<String> for KeyboardChild {
    fn from(s: String) -> Self {
        KeyboardChild::Text(s.into())
    }
}
impl From<CowStr> for KeyboardChild {
    fn from(s: CowStr) -> Self {
        KeyboardChild::Text(s)
    }
}
impl std::fmt::Debug for KeyboardChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Button(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Code(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Data(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Image(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Input(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Label(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Link(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Map(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Object(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Output(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Script(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Select(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Small(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Span(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Template(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Time(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Video(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            KeyboardChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for KeyboardChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Audio(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            KeyboardChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            KeyboardChild::Bold(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Button(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Cite(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Code(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Custom(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Data(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::DataList(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Definition(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Embed(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Image(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Input(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Italic(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Label(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Link(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Map(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Mark(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Meter(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Object(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Output(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Picture(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Progress(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Quote(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Sample(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Script(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Select(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Slot(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Small(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Span(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Strong(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Template(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Time(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Underline(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Variable(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Video(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            KeyboardChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
