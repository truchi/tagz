// 🤖 This file is generated!

use crate::*;
/// The `<cite>` element's children.
#[derive(Clone)]
pub enum CiteChild {
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
impl From<Abbreviation> for CiteChild {
    fn from(child: Abbreviation) -> Self {
        CiteChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for CiteChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        CiteChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for CiteChild {
    fn from(child: Anchor) -> Self {
        CiteChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for CiteChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        CiteChild::Anchor(builder.build())
    }
}
impl From<Audio> for CiteChild {
    fn from(child: Audio) -> Self {
        CiteChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for CiteChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        CiteChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for CiteChild {
    fn from(child: BidirectionalIsolate) -> Self {
        CiteChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for CiteChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        CiteChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for CiteChild {
    fn from(child: BidirectionalOverride) -> Self {
        CiteChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for CiteChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        CiteChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for CiteChild {
    fn from(child: Bold) -> Self {
        CiteChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for CiteChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        CiteChild::Bold(builder.build())
    }
}
impl From<Button> for CiteChild {
    fn from(child: Button) -> Self {
        CiteChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for CiteChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        CiteChild::Button(builder.build())
    }
}
impl From<Canvas> for CiteChild {
    fn from(child: Canvas) -> Self {
        CiteChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for CiteChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        CiteChild::Canvas(builder.build())
    }
}
impl From<Cite> for CiteChild {
    fn from(child: Cite) -> Self {
        CiteChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for CiteChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        CiteChild::Cite(builder.build())
    }
}
impl From<Code> for CiteChild {
    fn from(child: Code) -> Self {
        CiteChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for CiteChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        CiteChild::Code(builder.build())
    }
}
impl From<Custom> for CiteChild {
    fn from(child: Custom) -> Self {
        CiteChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for CiteChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        CiteChild::Custom(builder.build())
    }
}
impl From<Data> for CiteChild {
    fn from(child: Data) -> Self {
        CiteChild::Data(child)
    }
}
impl From<builders::DataBuilder> for CiteChild {
    fn from(builder: builders::DataBuilder) -> Self {
        CiteChild::Data(builder.build())
    }
}
impl From<DataList> for CiteChild {
    fn from(child: DataList) -> Self {
        CiteChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for CiteChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        CiteChild::DataList(builder.build())
    }
}
impl From<Definition> for CiteChild {
    fn from(child: Definition) -> Self {
        CiteChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for CiteChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        CiteChild::Definition(builder.build())
    }
}
impl From<Deleted> for CiteChild {
    fn from(child: Deleted) -> Self {
        CiteChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for CiteChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        CiteChild::Deleted(builder.build())
    }
}
impl From<Embed> for CiteChild {
    fn from(child: Embed) -> Self {
        CiteChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for CiteChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        CiteChild::Embed(builder.build())
    }
}
impl From<Emphasis> for CiteChild {
    fn from(child: Emphasis) -> Self {
        CiteChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for CiteChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        CiteChild::Emphasis(builder.build())
    }
}
impl From<Image> for CiteChild {
    fn from(child: Image) -> Self {
        CiteChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for CiteChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        CiteChild::Image(builder.build())
    }
}
impl From<InlineFrame> for CiteChild {
    fn from(child: InlineFrame) -> Self {
        CiteChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for CiteChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        CiteChild::InlineFrame(builder.build())
    }
}
impl From<Input> for CiteChild {
    fn from(child: Input) -> Self {
        CiteChild::Input(child)
    }
}
impl From<builders::InputBuilder> for CiteChild {
    fn from(builder: builders::InputBuilder) -> Self {
        CiteChild::Input(builder.build())
    }
}
impl From<Inserted> for CiteChild {
    fn from(child: Inserted) -> Self {
        CiteChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for CiteChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        CiteChild::Inserted(builder.build())
    }
}
impl From<Italic> for CiteChild {
    fn from(child: Italic) -> Self {
        CiteChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for CiteChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        CiteChild::Italic(builder.build())
    }
}
impl From<Keyboard> for CiteChild {
    fn from(child: Keyboard) -> Self {
        CiteChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for CiteChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        CiteChild::Keyboard(builder.build())
    }
}
impl From<Label> for CiteChild {
    fn from(child: Label) -> Self {
        CiteChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for CiteChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        CiteChild::Label(builder.build())
    }
}
impl From<LineBreak> for CiteChild {
    fn from(child: LineBreak) -> Self {
        CiteChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for CiteChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        CiteChild::LineBreak(builder.build())
    }
}
impl From<Link> for CiteChild {
    fn from(child: Link) -> Self {
        CiteChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for CiteChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        CiteChild::Link(builder.build())
    }
}
impl From<Map> for CiteChild {
    fn from(child: Map) -> Self {
        CiteChild::Map(child)
    }
}
impl From<builders::MapBuilder> for CiteChild {
    fn from(builder: builders::MapBuilder) -> Self {
        CiteChild::Map(builder.build())
    }
}
impl From<MapArea> for CiteChild {
    fn from(child: MapArea) -> Self {
        CiteChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for CiteChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        CiteChild::MapArea(builder.build())
    }
}
impl From<Mark> for CiteChild {
    fn from(child: Mark) -> Self {
        CiteChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for CiteChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        CiteChild::Mark(builder.build())
    }
}
impl From<Metadata> for CiteChild {
    fn from(child: Metadata) -> Self {
        CiteChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for CiteChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        CiteChild::Metadata(builder.build())
    }
}
impl From<Meter> for CiteChild {
    fn from(child: Meter) -> Self {
        CiteChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for CiteChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        CiteChild::Meter(builder.build())
    }
}
impl From<NoScript> for CiteChild {
    fn from(child: NoScript) -> Self {
        CiteChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for CiteChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        CiteChild::NoScript(builder.build())
    }
}
impl From<Object> for CiteChild {
    fn from(child: Object) -> Self {
        CiteChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for CiteChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        CiteChild::Object(builder.build())
    }
}
impl From<Output> for CiteChild {
    fn from(child: Output) -> Self {
        CiteChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for CiteChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        CiteChild::Output(builder.build())
    }
}
impl From<Picture> for CiteChild {
    fn from(child: Picture) -> Self {
        CiteChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for CiteChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        CiteChild::Picture(builder.build())
    }
}
impl From<Progress> for CiteChild {
    fn from(child: Progress) -> Self {
        CiteChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for CiteChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        CiteChild::Progress(builder.build())
    }
}
impl From<Quote> for CiteChild {
    fn from(child: Quote) -> Self {
        CiteChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for CiteChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        CiteChild::Quote(builder.build())
    }
}
impl From<Ruby> for CiteChild {
    fn from(child: Ruby) -> Self {
        CiteChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for CiteChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        CiteChild::Ruby(builder.build())
    }
}
impl From<Sample> for CiteChild {
    fn from(child: Sample) -> Self {
        CiteChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for CiteChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        CiteChild::Sample(builder.build())
    }
}
impl From<Script> for CiteChild {
    fn from(child: Script) -> Self {
        CiteChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for CiteChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        CiteChild::Script(builder.build())
    }
}
impl From<Select> for CiteChild {
    fn from(child: Select) -> Self {
        CiteChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for CiteChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        CiteChild::Select(builder.build())
    }
}
impl From<Slot> for CiteChild {
    fn from(child: Slot) -> Self {
        CiteChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for CiteChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        CiteChild::Slot(builder.build())
    }
}
impl From<Small> for CiteChild {
    fn from(child: Small) -> Self {
        CiteChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for CiteChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        CiteChild::Small(builder.build())
    }
}
impl From<Span> for CiteChild {
    fn from(child: Span) -> Self {
        CiteChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for CiteChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        CiteChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for CiteChild {
    fn from(child: StrikeThrough) -> Self {
        CiteChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for CiteChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        CiteChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for CiteChild {
    fn from(child: Strong) -> Self {
        CiteChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for CiteChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        CiteChild::Strong(builder.build())
    }
}
impl From<SubScript> for CiteChild {
    fn from(child: SubScript) -> Self {
        CiteChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for CiteChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        CiteChild::SubScript(builder.build())
    }
}
impl From<SupScript> for CiteChild {
    fn from(child: SupScript) -> Self {
        CiteChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for CiteChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        CiteChild::SupScript(builder.build())
    }
}
impl From<Template> for CiteChild {
    fn from(child: Template) -> Self {
        CiteChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for CiteChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        CiteChild::Template(builder.build())
    }
}
impl From<TextArea> for CiteChild {
    fn from(child: TextArea) -> Self {
        CiteChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for CiteChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        CiteChild::TextArea(builder.build())
    }
}
impl From<Time> for CiteChild {
    fn from(child: Time) -> Self {
        CiteChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for CiteChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        CiteChild::Time(builder.build())
    }
}
impl From<Underline> for CiteChild {
    fn from(child: Underline) -> Self {
        CiteChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for CiteChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        CiteChild::Underline(builder.build())
    }
}
impl From<Variable> for CiteChild {
    fn from(child: Variable) -> Self {
        CiteChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for CiteChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        CiteChild::Variable(builder.build())
    }
}
impl From<Video> for CiteChild {
    fn from(child: Video) -> Self {
        CiteChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for CiteChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        CiteChild::Video(builder.build())
    }
}
impl From<WordBreak> for CiteChild {
    fn from(child: WordBreak) -> Self {
        CiteChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for CiteChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        CiteChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for CiteChild {
    fn from(s: &'static str) -> Self {
        CiteChild::Text(s.into())
    }
}
impl From<String> for CiteChild {
    fn from(s: String) -> Self {
        CiteChild::Text(s.into())
    }
}
impl From<CowStr> for CiteChild {
    fn from(s: CowStr) -> Self {
        CiteChild::Text(s)
    }
}
impl std::fmt::Debug for CiteChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CiteChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Button(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Code(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Data(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Image(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Input(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Label(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Link(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Map(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Object(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Output(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Script(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Select(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Small(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Span(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Template(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Time(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Video(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            CiteChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for CiteChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CiteChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Audio(child) => std::fmt::Display::fmt(child, f),
            CiteChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            CiteChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Bold(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Button(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Cite(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Code(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Custom(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Data(child) => std::fmt::Display::fmt(child, f),
            CiteChild::DataList(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Definition(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Embed(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Image(child) => std::fmt::Display::fmt(child, f),
            CiteChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Input(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Italic(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Label(child) => std::fmt::Display::fmt(child, f),
            CiteChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Link(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Map(child) => std::fmt::Display::fmt(child, f),
            CiteChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Mark(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Meter(child) => std::fmt::Display::fmt(child, f),
            CiteChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Object(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Output(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Picture(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Progress(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Quote(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Sample(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Script(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Select(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Slot(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Small(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Span(child) => std::fmt::Display::fmt(child, f),
            CiteChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Strong(child) => std::fmt::Display::fmt(child, f),
            CiteChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            CiteChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Template(child) => std::fmt::Display::fmt(child, f),
            CiteChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Time(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Underline(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Variable(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Video(child) => std::fmt::Display::fmt(child, f),
            CiteChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            CiteChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
