// 🤖 This file is generated!

use crate::*;
/// The `<code>` element's children.
#[derive(Clone)]
pub enum CodeChild {
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
impl From<Abbreviation> for CodeChild {
    fn from(child: Abbreviation) -> Self {
        CodeChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for CodeChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        CodeChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for CodeChild {
    fn from(child: Anchor) -> Self {
        CodeChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for CodeChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        CodeChild::Anchor(builder.build())
    }
}
impl From<Audio> for CodeChild {
    fn from(child: Audio) -> Self {
        CodeChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for CodeChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        CodeChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for CodeChild {
    fn from(child: BidirectionalIsolate) -> Self {
        CodeChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for CodeChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        CodeChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for CodeChild {
    fn from(child: BidirectionalOverride) -> Self {
        CodeChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for CodeChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        CodeChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for CodeChild {
    fn from(child: Bold) -> Self {
        CodeChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for CodeChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        CodeChild::Bold(builder.build())
    }
}
impl From<Button> for CodeChild {
    fn from(child: Button) -> Self {
        CodeChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for CodeChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        CodeChild::Button(builder.build())
    }
}
impl From<Canvas> for CodeChild {
    fn from(child: Canvas) -> Self {
        CodeChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for CodeChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        CodeChild::Canvas(builder.build())
    }
}
impl From<Cite> for CodeChild {
    fn from(child: Cite) -> Self {
        CodeChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for CodeChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        CodeChild::Cite(builder.build())
    }
}
impl From<Code> for CodeChild {
    fn from(child: Code) -> Self {
        CodeChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for CodeChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        CodeChild::Code(builder.build())
    }
}
impl From<Custom> for CodeChild {
    fn from(child: Custom) -> Self {
        CodeChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for CodeChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        CodeChild::Custom(builder.build())
    }
}
impl From<Data> for CodeChild {
    fn from(child: Data) -> Self {
        CodeChild::Data(child)
    }
}
impl From<builders::DataBuilder> for CodeChild {
    fn from(builder: builders::DataBuilder) -> Self {
        CodeChild::Data(builder.build())
    }
}
impl From<DataList> for CodeChild {
    fn from(child: DataList) -> Self {
        CodeChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for CodeChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        CodeChild::DataList(builder.build())
    }
}
impl From<Definition> for CodeChild {
    fn from(child: Definition) -> Self {
        CodeChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for CodeChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        CodeChild::Definition(builder.build())
    }
}
impl From<Deleted> for CodeChild {
    fn from(child: Deleted) -> Self {
        CodeChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for CodeChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        CodeChild::Deleted(builder.build())
    }
}
impl From<Embed> for CodeChild {
    fn from(child: Embed) -> Self {
        CodeChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for CodeChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        CodeChild::Embed(builder.build())
    }
}
impl From<Emphasis> for CodeChild {
    fn from(child: Emphasis) -> Self {
        CodeChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for CodeChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        CodeChild::Emphasis(builder.build())
    }
}
impl From<Image> for CodeChild {
    fn from(child: Image) -> Self {
        CodeChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for CodeChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        CodeChild::Image(builder.build())
    }
}
impl From<InlineFrame> for CodeChild {
    fn from(child: InlineFrame) -> Self {
        CodeChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for CodeChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        CodeChild::InlineFrame(builder.build())
    }
}
impl From<Input> for CodeChild {
    fn from(child: Input) -> Self {
        CodeChild::Input(child)
    }
}
impl From<builders::InputBuilder> for CodeChild {
    fn from(builder: builders::InputBuilder) -> Self {
        CodeChild::Input(builder.build())
    }
}
impl From<Inserted> for CodeChild {
    fn from(child: Inserted) -> Self {
        CodeChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for CodeChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        CodeChild::Inserted(builder.build())
    }
}
impl From<Italic> for CodeChild {
    fn from(child: Italic) -> Self {
        CodeChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for CodeChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        CodeChild::Italic(builder.build())
    }
}
impl From<Keyboard> for CodeChild {
    fn from(child: Keyboard) -> Self {
        CodeChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for CodeChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        CodeChild::Keyboard(builder.build())
    }
}
impl From<Label> for CodeChild {
    fn from(child: Label) -> Self {
        CodeChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for CodeChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        CodeChild::Label(builder.build())
    }
}
impl From<LineBreak> for CodeChild {
    fn from(child: LineBreak) -> Self {
        CodeChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for CodeChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        CodeChild::LineBreak(builder.build())
    }
}
impl From<Link> for CodeChild {
    fn from(child: Link) -> Self {
        CodeChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for CodeChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        CodeChild::Link(builder.build())
    }
}
impl From<Map> for CodeChild {
    fn from(child: Map) -> Self {
        CodeChild::Map(child)
    }
}
impl From<builders::MapBuilder> for CodeChild {
    fn from(builder: builders::MapBuilder) -> Self {
        CodeChild::Map(builder.build())
    }
}
impl From<MapArea> for CodeChild {
    fn from(child: MapArea) -> Self {
        CodeChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for CodeChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        CodeChild::MapArea(builder.build())
    }
}
impl From<Mark> for CodeChild {
    fn from(child: Mark) -> Self {
        CodeChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for CodeChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        CodeChild::Mark(builder.build())
    }
}
impl From<Metadata> for CodeChild {
    fn from(child: Metadata) -> Self {
        CodeChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for CodeChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        CodeChild::Metadata(builder.build())
    }
}
impl From<Meter> for CodeChild {
    fn from(child: Meter) -> Self {
        CodeChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for CodeChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        CodeChild::Meter(builder.build())
    }
}
impl From<NoScript> for CodeChild {
    fn from(child: NoScript) -> Self {
        CodeChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for CodeChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        CodeChild::NoScript(builder.build())
    }
}
impl From<Object> for CodeChild {
    fn from(child: Object) -> Self {
        CodeChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for CodeChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        CodeChild::Object(builder.build())
    }
}
impl From<Output> for CodeChild {
    fn from(child: Output) -> Self {
        CodeChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for CodeChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        CodeChild::Output(builder.build())
    }
}
impl From<Picture> for CodeChild {
    fn from(child: Picture) -> Self {
        CodeChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for CodeChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        CodeChild::Picture(builder.build())
    }
}
impl From<Progress> for CodeChild {
    fn from(child: Progress) -> Self {
        CodeChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for CodeChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        CodeChild::Progress(builder.build())
    }
}
impl From<Quote> for CodeChild {
    fn from(child: Quote) -> Self {
        CodeChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for CodeChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        CodeChild::Quote(builder.build())
    }
}
impl From<Ruby> for CodeChild {
    fn from(child: Ruby) -> Self {
        CodeChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for CodeChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        CodeChild::Ruby(builder.build())
    }
}
impl From<Sample> for CodeChild {
    fn from(child: Sample) -> Self {
        CodeChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for CodeChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        CodeChild::Sample(builder.build())
    }
}
impl From<Script> for CodeChild {
    fn from(child: Script) -> Self {
        CodeChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for CodeChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        CodeChild::Script(builder.build())
    }
}
impl From<Select> for CodeChild {
    fn from(child: Select) -> Self {
        CodeChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for CodeChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        CodeChild::Select(builder.build())
    }
}
impl From<Slot> for CodeChild {
    fn from(child: Slot) -> Self {
        CodeChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for CodeChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        CodeChild::Slot(builder.build())
    }
}
impl From<Small> for CodeChild {
    fn from(child: Small) -> Self {
        CodeChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for CodeChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        CodeChild::Small(builder.build())
    }
}
impl From<Span> for CodeChild {
    fn from(child: Span) -> Self {
        CodeChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for CodeChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        CodeChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for CodeChild {
    fn from(child: StrikeThrough) -> Self {
        CodeChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for CodeChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        CodeChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for CodeChild {
    fn from(child: Strong) -> Self {
        CodeChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for CodeChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        CodeChild::Strong(builder.build())
    }
}
impl From<SubScript> for CodeChild {
    fn from(child: SubScript) -> Self {
        CodeChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for CodeChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        CodeChild::SubScript(builder.build())
    }
}
impl From<SupScript> for CodeChild {
    fn from(child: SupScript) -> Self {
        CodeChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for CodeChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        CodeChild::SupScript(builder.build())
    }
}
impl From<Template> for CodeChild {
    fn from(child: Template) -> Self {
        CodeChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for CodeChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        CodeChild::Template(builder.build())
    }
}
impl From<TextArea> for CodeChild {
    fn from(child: TextArea) -> Self {
        CodeChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for CodeChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        CodeChild::TextArea(builder.build())
    }
}
impl From<Time> for CodeChild {
    fn from(child: Time) -> Self {
        CodeChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for CodeChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        CodeChild::Time(builder.build())
    }
}
impl From<Underline> for CodeChild {
    fn from(child: Underline) -> Self {
        CodeChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for CodeChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        CodeChild::Underline(builder.build())
    }
}
impl From<Variable> for CodeChild {
    fn from(child: Variable) -> Self {
        CodeChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for CodeChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        CodeChild::Variable(builder.build())
    }
}
impl From<Video> for CodeChild {
    fn from(child: Video) -> Self {
        CodeChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for CodeChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        CodeChild::Video(builder.build())
    }
}
impl From<WordBreak> for CodeChild {
    fn from(child: WordBreak) -> Self {
        CodeChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for CodeChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        CodeChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for CodeChild {
    fn from(s: &'static str) -> Self {
        CodeChild::Text(s.into())
    }
}
impl From<String> for CodeChild {
    fn from(s: String) -> Self {
        CodeChild::Text(s.into())
    }
}
impl From<CowStr> for CodeChild {
    fn from(s: CowStr) -> Self {
        CodeChild::Text(s)
    }
}
impl std::fmt::Debug for CodeChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Button(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Code(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Data(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Image(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Input(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Label(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Link(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Map(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Object(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Output(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Script(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Select(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Small(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Span(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Template(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Time(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Video(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            CodeChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for CodeChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Audio(child) => std::fmt::Display::fmt(child, f),
            CodeChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            CodeChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Bold(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Button(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Cite(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Code(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Custom(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Data(child) => std::fmt::Display::fmt(child, f),
            CodeChild::DataList(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Definition(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Embed(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Image(child) => std::fmt::Display::fmt(child, f),
            CodeChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Input(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Italic(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Label(child) => std::fmt::Display::fmt(child, f),
            CodeChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Link(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Map(child) => std::fmt::Display::fmt(child, f),
            CodeChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Mark(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Meter(child) => std::fmt::Display::fmt(child, f),
            CodeChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Object(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Output(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Picture(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Progress(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Quote(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Sample(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Script(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Select(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Slot(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Small(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Span(child) => std::fmt::Display::fmt(child, f),
            CodeChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Strong(child) => std::fmt::Display::fmt(child, f),
            CodeChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            CodeChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Template(child) => std::fmt::Display::fmt(child, f),
            CodeChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Time(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Underline(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Variable(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Video(child) => std::fmt::Display::fmt(child, f),
            CodeChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            CodeChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
