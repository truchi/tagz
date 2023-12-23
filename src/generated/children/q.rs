// 🤖 This file is generated!

use crate::*;
/// The `<q>` element's children.
#[derive(Clone)]
pub enum QuoteChild {
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
impl From<Abbreviation> for QuoteChild {
    fn from(child: Abbreviation) -> Self {
        QuoteChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for QuoteChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        QuoteChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for QuoteChild {
    fn from(child: Anchor) -> Self {
        QuoteChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for QuoteChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        QuoteChild::Anchor(builder.build())
    }
}
impl From<Audio> for QuoteChild {
    fn from(child: Audio) -> Self {
        QuoteChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for QuoteChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        QuoteChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for QuoteChild {
    fn from(child: BidirectionalIsolate) -> Self {
        QuoteChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for QuoteChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        QuoteChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for QuoteChild {
    fn from(child: BidirectionalOverride) -> Self {
        QuoteChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for QuoteChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        QuoteChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for QuoteChild {
    fn from(child: Bold) -> Self {
        QuoteChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for QuoteChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        QuoteChild::Bold(builder.build())
    }
}
impl From<Button> for QuoteChild {
    fn from(child: Button) -> Self {
        QuoteChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for QuoteChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        QuoteChild::Button(builder.build())
    }
}
impl From<Canvas> for QuoteChild {
    fn from(child: Canvas) -> Self {
        QuoteChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for QuoteChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        QuoteChild::Canvas(builder.build())
    }
}
impl From<Cite> for QuoteChild {
    fn from(child: Cite) -> Self {
        QuoteChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for QuoteChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        QuoteChild::Cite(builder.build())
    }
}
impl From<Code> for QuoteChild {
    fn from(child: Code) -> Self {
        QuoteChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for QuoteChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        QuoteChild::Code(builder.build())
    }
}
impl From<Custom> for QuoteChild {
    fn from(child: Custom) -> Self {
        QuoteChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for QuoteChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        QuoteChild::Custom(builder.build())
    }
}
impl From<Data> for QuoteChild {
    fn from(child: Data) -> Self {
        QuoteChild::Data(child)
    }
}
impl From<builders::DataBuilder> for QuoteChild {
    fn from(builder: builders::DataBuilder) -> Self {
        QuoteChild::Data(builder.build())
    }
}
impl From<DataList> for QuoteChild {
    fn from(child: DataList) -> Self {
        QuoteChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for QuoteChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        QuoteChild::DataList(builder.build())
    }
}
impl From<Definition> for QuoteChild {
    fn from(child: Definition) -> Self {
        QuoteChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for QuoteChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        QuoteChild::Definition(builder.build())
    }
}
impl From<Deleted> for QuoteChild {
    fn from(child: Deleted) -> Self {
        QuoteChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for QuoteChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        QuoteChild::Deleted(builder.build())
    }
}
impl From<Embed> for QuoteChild {
    fn from(child: Embed) -> Self {
        QuoteChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for QuoteChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        QuoteChild::Embed(builder.build())
    }
}
impl From<Emphasis> for QuoteChild {
    fn from(child: Emphasis) -> Self {
        QuoteChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for QuoteChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        QuoteChild::Emphasis(builder.build())
    }
}
impl From<Image> for QuoteChild {
    fn from(child: Image) -> Self {
        QuoteChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for QuoteChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        QuoteChild::Image(builder.build())
    }
}
impl From<InlineFrame> for QuoteChild {
    fn from(child: InlineFrame) -> Self {
        QuoteChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for QuoteChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        QuoteChild::InlineFrame(builder.build())
    }
}
impl From<Input> for QuoteChild {
    fn from(child: Input) -> Self {
        QuoteChild::Input(child)
    }
}
impl From<builders::InputBuilder> for QuoteChild {
    fn from(builder: builders::InputBuilder) -> Self {
        QuoteChild::Input(builder.build())
    }
}
impl From<Inserted> for QuoteChild {
    fn from(child: Inserted) -> Self {
        QuoteChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for QuoteChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        QuoteChild::Inserted(builder.build())
    }
}
impl From<Italic> for QuoteChild {
    fn from(child: Italic) -> Self {
        QuoteChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for QuoteChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        QuoteChild::Italic(builder.build())
    }
}
impl From<Keyboard> for QuoteChild {
    fn from(child: Keyboard) -> Self {
        QuoteChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for QuoteChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        QuoteChild::Keyboard(builder.build())
    }
}
impl From<Label> for QuoteChild {
    fn from(child: Label) -> Self {
        QuoteChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for QuoteChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        QuoteChild::Label(builder.build())
    }
}
impl From<LineBreak> for QuoteChild {
    fn from(child: LineBreak) -> Self {
        QuoteChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for QuoteChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        QuoteChild::LineBreak(builder.build())
    }
}
impl From<Link> for QuoteChild {
    fn from(child: Link) -> Self {
        QuoteChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for QuoteChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        QuoteChild::Link(builder.build())
    }
}
impl From<Map> for QuoteChild {
    fn from(child: Map) -> Self {
        QuoteChild::Map(child)
    }
}
impl From<builders::MapBuilder> for QuoteChild {
    fn from(builder: builders::MapBuilder) -> Self {
        QuoteChild::Map(builder.build())
    }
}
impl From<MapArea> for QuoteChild {
    fn from(child: MapArea) -> Self {
        QuoteChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for QuoteChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        QuoteChild::MapArea(builder.build())
    }
}
impl From<Mark> for QuoteChild {
    fn from(child: Mark) -> Self {
        QuoteChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for QuoteChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        QuoteChild::Mark(builder.build())
    }
}
impl From<Metadata> for QuoteChild {
    fn from(child: Metadata) -> Self {
        QuoteChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for QuoteChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        QuoteChild::Metadata(builder.build())
    }
}
impl From<Meter> for QuoteChild {
    fn from(child: Meter) -> Self {
        QuoteChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for QuoteChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        QuoteChild::Meter(builder.build())
    }
}
impl From<NoScript> for QuoteChild {
    fn from(child: NoScript) -> Self {
        QuoteChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for QuoteChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        QuoteChild::NoScript(builder.build())
    }
}
impl From<Object> for QuoteChild {
    fn from(child: Object) -> Self {
        QuoteChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for QuoteChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        QuoteChild::Object(builder.build())
    }
}
impl From<Output> for QuoteChild {
    fn from(child: Output) -> Self {
        QuoteChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for QuoteChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        QuoteChild::Output(builder.build())
    }
}
impl From<Picture> for QuoteChild {
    fn from(child: Picture) -> Self {
        QuoteChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for QuoteChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        QuoteChild::Picture(builder.build())
    }
}
impl From<Progress> for QuoteChild {
    fn from(child: Progress) -> Self {
        QuoteChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for QuoteChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        QuoteChild::Progress(builder.build())
    }
}
impl From<Quote> for QuoteChild {
    fn from(child: Quote) -> Self {
        QuoteChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for QuoteChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        QuoteChild::Quote(builder.build())
    }
}
impl From<Ruby> for QuoteChild {
    fn from(child: Ruby) -> Self {
        QuoteChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for QuoteChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        QuoteChild::Ruby(builder.build())
    }
}
impl From<Sample> for QuoteChild {
    fn from(child: Sample) -> Self {
        QuoteChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for QuoteChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        QuoteChild::Sample(builder.build())
    }
}
impl From<Script> for QuoteChild {
    fn from(child: Script) -> Self {
        QuoteChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for QuoteChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        QuoteChild::Script(builder.build())
    }
}
impl From<Select> for QuoteChild {
    fn from(child: Select) -> Self {
        QuoteChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for QuoteChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        QuoteChild::Select(builder.build())
    }
}
impl From<Slot> for QuoteChild {
    fn from(child: Slot) -> Self {
        QuoteChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for QuoteChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        QuoteChild::Slot(builder.build())
    }
}
impl From<Small> for QuoteChild {
    fn from(child: Small) -> Self {
        QuoteChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for QuoteChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        QuoteChild::Small(builder.build())
    }
}
impl From<Span> for QuoteChild {
    fn from(child: Span) -> Self {
        QuoteChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for QuoteChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        QuoteChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for QuoteChild {
    fn from(child: StrikeThrough) -> Self {
        QuoteChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for QuoteChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        QuoteChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for QuoteChild {
    fn from(child: Strong) -> Self {
        QuoteChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for QuoteChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        QuoteChild::Strong(builder.build())
    }
}
impl From<SubScript> for QuoteChild {
    fn from(child: SubScript) -> Self {
        QuoteChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for QuoteChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        QuoteChild::SubScript(builder.build())
    }
}
impl From<SupScript> for QuoteChild {
    fn from(child: SupScript) -> Self {
        QuoteChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for QuoteChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        QuoteChild::SupScript(builder.build())
    }
}
impl From<Template> for QuoteChild {
    fn from(child: Template) -> Self {
        QuoteChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for QuoteChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        QuoteChild::Template(builder.build())
    }
}
impl From<TextArea> for QuoteChild {
    fn from(child: TextArea) -> Self {
        QuoteChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for QuoteChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        QuoteChild::TextArea(builder.build())
    }
}
impl From<Time> for QuoteChild {
    fn from(child: Time) -> Self {
        QuoteChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for QuoteChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        QuoteChild::Time(builder.build())
    }
}
impl From<Underline> for QuoteChild {
    fn from(child: Underline) -> Self {
        QuoteChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for QuoteChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        QuoteChild::Underline(builder.build())
    }
}
impl From<Variable> for QuoteChild {
    fn from(child: Variable) -> Self {
        QuoteChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for QuoteChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        QuoteChild::Variable(builder.build())
    }
}
impl From<Video> for QuoteChild {
    fn from(child: Video) -> Self {
        QuoteChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for QuoteChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        QuoteChild::Video(builder.build())
    }
}
impl From<WordBreak> for QuoteChild {
    fn from(child: WordBreak) -> Self {
        QuoteChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for QuoteChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        QuoteChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for QuoteChild {
    fn from(s: &'static str) -> Self {
        QuoteChild::Text(s.into())
    }
}
impl From<String> for QuoteChild {
    fn from(s: String) -> Self {
        QuoteChild::Text(s.into())
    }
}
impl From<CowStr> for QuoteChild {
    fn from(s: CowStr) -> Self {
        QuoteChild::Text(s)
    }
}
impl std::fmt::Debug for QuoteChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Button(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Code(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Data(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Image(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Input(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Label(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Link(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Map(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Object(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Output(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Script(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Select(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Small(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Span(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Template(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Time(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Video(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            QuoteChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for QuoteChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Audio(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Bold(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Button(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Cite(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Code(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Custom(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Data(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::DataList(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Definition(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Embed(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Image(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Input(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Italic(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Label(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Link(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Map(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Mark(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Meter(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Object(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Output(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Picture(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Progress(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Quote(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Sample(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Script(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Select(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Slot(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Small(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Span(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Strong(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Template(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Time(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Underline(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Variable(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Video(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            QuoteChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
