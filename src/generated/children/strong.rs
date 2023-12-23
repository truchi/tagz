// 🤖 This file is generated!

use crate::*;
/// The `<strong>` element's children.
#[derive(Clone)]
pub enum StrongChild {
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
impl From<Abbreviation> for StrongChild {
    fn from(child: Abbreviation) -> Self {
        StrongChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for StrongChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        StrongChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for StrongChild {
    fn from(child: Anchor) -> Self {
        StrongChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for StrongChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        StrongChild::Anchor(builder.build())
    }
}
impl From<Audio> for StrongChild {
    fn from(child: Audio) -> Self {
        StrongChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for StrongChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        StrongChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for StrongChild {
    fn from(child: BidirectionalIsolate) -> Self {
        StrongChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for StrongChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        StrongChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for StrongChild {
    fn from(child: BidirectionalOverride) -> Self {
        StrongChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for StrongChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        StrongChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for StrongChild {
    fn from(child: Bold) -> Self {
        StrongChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for StrongChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        StrongChild::Bold(builder.build())
    }
}
impl From<Button> for StrongChild {
    fn from(child: Button) -> Self {
        StrongChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for StrongChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        StrongChild::Button(builder.build())
    }
}
impl From<Canvas> for StrongChild {
    fn from(child: Canvas) -> Self {
        StrongChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for StrongChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        StrongChild::Canvas(builder.build())
    }
}
impl From<Cite> for StrongChild {
    fn from(child: Cite) -> Self {
        StrongChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for StrongChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        StrongChild::Cite(builder.build())
    }
}
impl From<Code> for StrongChild {
    fn from(child: Code) -> Self {
        StrongChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for StrongChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        StrongChild::Code(builder.build())
    }
}
impl From<Custom> for StrongChild {
    fn from(child: Custom) -> Self {
        StrongChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for StrongChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        StrongChild::Custom(builder.build())
    }
}
impl From<Data> for StrongChild {
    fn from(child: Data) -> Self {
        StrongChild::Data(child)
    }
}
impl From<builders::DataBuilder> for StrongChild {
    fn from(builder: builders::DataBuilder) -> Self {
        StrongChild::Data(builder.build())
    }
}
impl From<DataList> for StrongChild {
    fn from(child: DataList) -> Self {
        StrongChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for StrongChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        StrongChild::DataList(builder.build())
    }
}
impl From<Definition> for StrongChild {
    fn from(child: Definition) -> Self {
        StrongChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for StrongChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        StrongChild::Definition(builder.build())
    }
}
impl From<Deleted> for StrongChild {
    fn from(child: Deleted) -> Self {
        StrongChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for StrongChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        StrongChild::Deleted(builder.build())
    }
}
impl From<Embed> for StrongChild {
    fn from(child: Embed) -> Self {
        StrongChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for StrongChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        StrongChild::Embed(builder.build())
    }
}
impl From<Emphasis> for StrongChild {
    fn from(child: Emphasis) -> Self {
        StrongChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for StrongChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        StrongChild::Emphasis(builder.build())
    }
}
impl From<Image> for StrongChild {
    fn from(child: Image) -> Self {
        StrongChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for StrongChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        StrongChild::Image(builder.build())
    }
}
impl From<InlineFrame> for StrongChild {
    fn from(child: InlineFrame) -> Self {
        StrongChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for StrongChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        StrongChild::InlineFrame(builder.build())
    }
}
impl From<Input> for StrongChild {
    fn from(child: Input) -> Self {
        StrongChild::Input(child)
    }
}
impl From<builders::InputBuilder> for StrongChild {
    fn from(builder: builders::InputBuilder) -> Self {
        StrongChild::Input(builder.build())
    }
}
impl From<Inserted> for StrongChild {
    fn from(child: Inserted) -> Self {
        StrongChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for StrongChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        StrongChild::Inserted(builder.build())
    }
}
impl From<Italic> for StrongChild {
    fn from(child: Italic) -> Self {
        StrongChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for StrongChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        StrongChild::Italic(builder.build())
    }
}
impl From<Keyboard> for StrongChild {
    fn from(child: Keyboard) -> Self {
        StrongChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for StrongChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        StrongChild::Keyboard(builder.build())
    }
}
impl From<Label> for StrongChild {
    fn from(child: Label) -> Self {
        StrongChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for StrongChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        StrongChild::Label(builder.build())
    }
}
impl From<LineBreak> for StrongChild {
    fn from(child: LineBreak) -> Self {
        StrongChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for StrongChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        StrongChild::LineBreak(builder.build())
    }
}
impl From<Link> for StrongChild {
    fn from(child: Link) -> Self {
        StrongChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for StrongChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        StrongChild::Link(builder.build())
    }
}
impl From<Map> for StrongChild {
    fn from(child: Map) -> Self {
        StrongChild::Map(child)
    }
}
impl From<builders::MapBuilder> for StrongChild {
    fn from(builder: builders::MapBuilder) -> Self {
        StrongChild::Map(builder.build())
    }
}
impl From<MapArea> for StrongChild {
    fn from(child: MapArea) -> Self {
        StrongChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for StrongChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        StrongChild::MapArea(builder.build())
    }
}
impl From<Mark> for StrongChild {
    fn from(child: Mark) -> Self {
        StrongChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for StrongChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        StrongChild::Mark(builder.build())
    }
}
impl From<Metadata> for StrongChild {
    fn from(child: Metadata) -> Self {
        StrongChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for StrongChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        StrongChild::Metadata(builder.build())
    }
}
impl From<Meter> for StrongChild {
    fn from(child: Meter) -> Self {
        StrongChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for StrongChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        StrongChild::Meter(builder.build())
    }
}
impl From<NoScript> for StrongChild {
    fn from(child: NoScript) -> Self {
        StrongChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for StrongChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        StrongChild::NoScript(builder.build())
    }
}
impl From<Object> for StrongChild {
    fn from(child: Object) -> Self {
        StrongChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for StrongChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        StrongChild::Object(builder.build())
    }
}
impl From<Output> for StrongChild {
    fn from(child: Output) -> Self {
        StrongChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for StrongChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        StrongChild::Output(builder.build())
    }
}
impl From<Picture> for StrongChild {
    fn from(child: Picture) -> Self {
        StrongChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for StrongChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        StrongChild::Picture(builder.build())
    }
}
impl From<Progress> for StrongChild {
    fn from(child: Progress) -> Self {
        StrongChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for StrongChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        StrongChild::Progress(builder.build())
    }
}
impl From<Quote> for StrongChild {
    fn from(child: Quote) -> Self {
        StrongChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for StrongChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        StrongChild::Quote(builder.build())
    }
}
impl From<Ruby> for StrongChild {
    fn from(child: Ruby) -> Self {
        StrongChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for StrongChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        StrongChild::Ruby(builder.build())
    }
}
impl From<Sample> for StrongChild {
    fn from(child: Sample) -> Self {
        StrongChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for StrongChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        StrongChild::Sample(builder.build())
    }
}
impl From<Script> for StrongChild {
    fn from(child: Script) -> Self {
        StrongChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for StrongChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        StrongChild::Script(builder.build())
    }
}
impl From<Select> for StrongChild {
    fn from(child: Select) -> Self {
        StrongChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for StrongChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        StrongChild::Select(builder.build())
    }
}
impl From<Slot> for StrongChild {
    fn from(child: Slot) -> Self {
        StrongChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for StrongChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        StrongChild::Slot(builder.build())
    }
}
impl From<Small> for StrongChild {
    fn from(child: Small) -> Self {
        StrongChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for StrongChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        StrongChild::Small(builder.build())
    }
}
impl From<Span> for StrongChild {
    fn from(child: Span) -> Self {
        StrongChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for StrongChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        StrongChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for StrongChild {
    fn from(child: StrikeThrough) -> Self {
        StrongChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for StrongChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        StrongChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for StrongChild {
    fn from(child: Strong) -> Self {
        StrongChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for StrongChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        StrongChild::Strong(builder.build())
    }
}
impl From<SubScript> for StrongChild {
    fn from(child: SubScript) -> Self {
        StrongChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for StrongChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        StrongChild::SubScript(builder.build())
    }
}
impl From<SupScript> for StrongChild {
    fn from(child: SupScript) -> Self {
        StrongChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for StrongChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        StrongChild::SupScript(builder.build())
    }
}
impl From<Template> for StrongChild {
    fn from(child: Template) -> Self {
        StrongChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for StrongChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        StrongChild::Template(builder.build())
    }
}
impl From<TextArea> for StrongChild {
    fn from(child: TextArea) -> Self {
        StrongChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for StrongChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        StrongChild::TextArea(builder.build())
    }
}
impl From<Time> for StrongChild {
    fn from(child: Time) -> Self {
        StrongChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for StrongChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        StrongChild::Time(builder.build())
    }
}
impl From<Underline> for StrongChild {
    fn from(child: Underline) -> Self {
        StrongChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for StrongChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        StrongChild::Underline(builder.build())
    }
}
impl From<Variable> for StrongChild {
    fn from(child: Variable) -> Self {
        StrongChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for StrongChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        StrongChild::Variable(builder.build())
    }
}
impl From<Video> for StrongChild {
    fn from(child: Video) -> Self {
        StrongChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for StrongChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        StrongChild::Video(builder.build())
    }
}
impl From<WordBreak> for StrongChild {
    fn from(child: WordBreak) -> Self {
        StrongChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for StrongChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        StrongChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for StrongChild {
    fn from(s: &'static str) -> Self {
        StrongChild::Text(s.into())
    }
}
impl From<String> for StrongChild {
    fn from(s: String) -> Self {
        StrongChild::Text(s.into())
    }
}
impl From<CowStr> for StrongChild {
    fn from(s: CowStr) -> Self {
        StrongChild::Text(s)
    }
}
impl std::fmt::Debug for StrongChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrongChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Button(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Code(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Data(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Image(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Input(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Label(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Link(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Map(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Object(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Output(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Script(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Select(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Small(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Span(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Template(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Time(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Video(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            StrongChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for StrongChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrongChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Audio(child) => std::fmt::Display::fmt(child, f),
            StrongChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            StrongChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Bold(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Button(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Cite(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Code(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Custom(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Data(child) => std::fmt::Display::fmt(child, f),
            StrongChild::DataList(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Definition(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Embed(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Image(child) => std::fmt::Display::fmt(child, f),
            StrongChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Input(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Italic(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Label(child) => std::fmt::Display::fmt(child, f),
            StrongChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Link(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Map(child) => std::fmt::Display::fmt(child, f),
            StrongChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Mark(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Meter(child) => std::fmt::Display::fmt(child, f),
            StrongChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Object(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Output(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Picture(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Progress(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Quote(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Sample(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Script(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Select(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Slot(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Small(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Span(child) => std::fmt::Display::fmt(child, f),
            StrongChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Strong(child) => std::fmt::Display::fmt(child, f),
            StrongChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            StrongChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Template(child) => std::fmt::Display::fmt(child, f),
            StrongChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Time(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Underline(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Variable(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Video(child) => std::fmt::Display::fmt(child, f),
            StrongChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            StrongChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
