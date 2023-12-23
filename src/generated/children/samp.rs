// 🤖 This file is generated!

use crate::*;
/// The `<samp>` element's children.
#[derive(Clone)]
pub enum SampleChild {
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
impl From<Abbreviation> for SampleChild {
    fn from(child: Abbreviation) -> Self {
        SampleChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for SampleChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        SampleChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for SampleChild {
    fn from(child: Anchor) -> Self {
        SampleChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for SampleChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        SampleChild::Anchor(builder.build())
    }
}
impl From<Audio> for SampleChild {
    fn from(child: Audio) -> Self {
        SampleChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for SampleChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        SampleChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for SampleChild {
    fn from(child: BidirectionalIsolate) -> Self {
        SampleChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for SampleChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        SampleChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for SampleChild {
    fn from(child: BidirectionalOverride) -> Self {
        SampleChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for SampleChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        SampleChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for SampleChild {
    fn from(child: Bold) -> Self {
        SampleChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for SampleChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        SampleChild::Bold(builder.build())
    }
}
impl From<Button> for SampleChild {
    fn from(child: Button) -> Self {
        SampleChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for SampleChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        SampleChild::Button(builder.build())
    }
}
impl From<Canvas> for SampleChild {
    fn from(child: Canvas) -> Self {
        SampleChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for SampleChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        SampleChild::Canvas(builder.build())
    }
}
impl From<Cite> for SampleChild {
    fn from(child: Cite) -> Self {
        SampleChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for SampleChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        SampleChild::Cite(builder.build())
    }
}
impl From<Code> for SampleChild {
    fn from(child: Code) -> Self {
        SampleChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for SampleChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        SampleChild::Code(builder.build())
    }
}
impl From<Custom> for SampleChild {
    fn from(child: Custom) -> Self {
        SampleChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for SampleChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        SampleChild::Custom(builder.build())
    }
}
impl From<Data> for SampleChild {
    fn from(child: Data) -> Self {
        SampleChild::Data(child)
    }
}
impl From<builders::DataBuilder> for SampleChild {
    fn from(builder: builders::DataBuilder) -> Self {
        SampleChild::Data(builder.build())
    }
}
impl From<DataList> for SampleChild {
    fn from(child: DataList) -> Self {
        SampleChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for SampleChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        SampleChild::DataList(builder.build())
    }
}
impl From<Definition> for SampleChild {
    fn from(child: Definition) -> Self {
        SampleChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for SampleChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        SampleChild::Definition(builder.build())
    }
}
impl From<Deleted> for SampleChild {
    fn from(child: Deleted) -> Self {
        SampleChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for SampleChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        SampleChild::Deleted(builder.build())
    }
}
impl From<Embed> for SampleChild {
    fn from(child: Embed) -> Self {
        SampleChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for SampleChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        SampleChild::Embed(builder.build())
    }
}
impl From<Emphasis> for SampleChild {
    fn from(child: Emphasis) -> Self {
        SampleChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for SampleChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        SampleChild::Emphasis(builder.build())
    }
}
impl From<Image> for SampleChild {
    fn from(child: Image) -> Self {
        SampleChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for SampleChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        SampleChild::Image(builder.build())
    }
}
impl From<InlineFrame> for SampleChild {
    fn from(child: InlineFrame) -> Self {
        SampleChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for SampleChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        SampleChild::InlineFrame(builder.build())
    }
}
impl From<Input> for SampleChild {
    fn from(child: Input) -> Self {
        SampleChild::Input(child)
    }
}
impl From<builders::InputBuilder> for SampleChild {
    fn from(builder: builders::InputBuilder) -> Self {
        SampleChild::Input(builder.build())
    }
}
impl From<Inserted> for SampleChild {
    fn from(child: Inserted) -> Self {
        SampleChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for SampleChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        SampleChild::Inserted(builder.build())
    }
}
impl From<Italic> for SampleChild {
    fn from(child: Italic) -> Self {
        SampleChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for SampleChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        SampleChild::Italic(builder.build())
    }
}
impl From<Keyboard> for SampleChild {
    fn from(child: Keyboard) -> Self {
        SampleChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for SampleChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        SampleChild::Keyboard(builder.build())
    }
}
impl From<Label> for SampleChild {
    fn from(child: Label) -> Self {
        SampleChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for SampleChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        SampleChild::Label(builder.build())
    }
}
impl From<LineBreak> for SampleChild {
    fn from(child: LineBreak) -> Self {
        SampleChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for SampleChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        SampleChild::LineBreak(builder.build())
    }
}
impl From<Link> for SampleChild {
    fn from(child: Link) -> Self {
        SampleChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for SampleChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        SampleChild::Link(builder.build())
    }
}
impl From<Map> for SampleChild {
    fn from(child: Map) -> Self {
        SampleChild::Map(child)
    }
}
impl From<builders::MapBuilder> for SampleChild {
    fn from(builder: builders::MapBuilder) -> Self {
        SampleChild::Map(builder.build())
    }
}
impl From<MapArea> for SampleChild {
    fn from(child: MapArea) -> Self {
        SampleChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for SampleChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        SampleChild::MapArea(builder.build())
    }
}
impl From<Mark> for SampleChild {
    fn from(child: Mark) -> Self {
        SampleChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for SampleChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        SampleChild::Mark(builder.build())
    }
}
impl From<Metadata> for SampleChild {
    fn from(child: Metadata) -> Self {
        SampleChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for SampleChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        SampleChild::Metadata(builder.build())
    }
}
impl From<Meter> for SampleChild {
    fn from(child: Meter) -> Self {
        SampleChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for SampleChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        SampleChild::Meter(builder.build())
    }
}
impl From<NoScript> for SampleChild {
    fn from(child: NoScript) -> Self {
        SampleChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for SampleChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        SampleChild::NoScript(builder.build())
    }
}
impl From<Object> for SampleChild {
    fn from(child: Object) -> Self {
        SampleChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for SampleChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        SampleChild::Object(builder.build())
    }
}
impl From<Output> for SampleChild {
    fn from(child: Output) -> Self {
        SampleChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for SampleChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        SampleChild::Output(builder.build())
    }
}
impl From<Picture> for SampleChild {
    fn from(child: Picture) -> Self {
        SampleChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for SampleChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        SampleChild::Picture(builder.build())
    }
}
impl From<Progress> for SampleChild {
    fn from(child: Progress) -> Self {
        SampleChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for SampleChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        SampleChild::Progress(builder.build())
    }
}
impl From<Quote> for SampleChild {
    fn from(child: Quote) -> Self {
        SampleChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for SampleChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        SampleChild::Quote(builder.build())
    }
}
impl From<Ruby> for SampleChild {
    fn from(child: Ruby) -> Self {
        SampleChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for SampleChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        SampleChild::Ruby(builder.build())
    }
}
impl From<Sample> for SampleChild {
    fn from(child: Sample) -> Self {
        SampleChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for SampleChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        SampleChild::Sample(builder.build())
    }
}
impl From<Script> for SampleChild {
    fn from(child: Script) -> Self {
        SampleChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SampleChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SampleChild::Script(builder.build())
    }
}
impl From<Select> for SampleChild {
    fn from(child: Select) -> Self {
        SampleChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for SampleChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        SampleChild::Select(builder.build())
    }
}
impl From<Slot> for SampleChild {
    fn from(child: Slot) -> Self {
        SampleChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for SampleChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        SampleChild::Slot(builder.build())
    }
}
impl From<Small> for SampleChild {
    fn from(child: Small) -> Self {
        SampleChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for SampleChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        SampleChild::Small(builder.build())
    }
}
impl From<Span> for SampleChild {
    fn from(child: Span) -> Self {
        SampleChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for SampleChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        SampleChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for SampleChild {
    fn from(child: StrikeThrough) -> Self {
        SampleChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for SampleChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        SampleChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for SampleChild {
    fn from(child: Strong) -> Self {
        SampleChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for SampleChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        SampleChild::Strong(builder.build())
    }
}
impl From<SubScript> for SampleChild {
    fn from(child: SubScript) -> Self {
        SampleChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for SampleChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        SampleChild::SubScript(builder.build())
    }
}
impl From<SupScript> for SampleChild {
    fn from(child: SupScript) -> Self {
        SampleChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for SampleChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        SampleChild::SupScript(builder.build())
    }
}
impl From<Template> for SampleChild {
    fn from(child: Template) -> Self {
        SampleChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SampleChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SampleChild::Template(builder.build())
    }
}
impl From<TextArea> for SampleChild {
    fn from(child: TextArea) -> Self {
        SampleChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for SampleChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        SampleChild::TextArea(builder.build())
    }
}
impl From<Time> for SampleChild {
    fn from(child: Time) -> Self {
        SampleChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for SampleChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        SampleChild::Time(builder.build())
    }
}
impl From<Underline> for SampleChild {
    fn from(child: Underline) -> Self {
        SampleChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for SampleChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        SampleChild::Underline(builder.build())
    }
}
impl From<Variable> for SampleChild {
    fn from(child: Variable) -> Self {
        SampleChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for SampleChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        SampleChild::Variable(builder.build())
    }
}
impl From<Video> for SampleChild {
    fn from(child: Video) -> Self {
        SampleChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for SampleChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        SampleChild::Video(builder.build())
    }
}
impl From<WordBreak> for SampleChild {
    fn from(child: WordBreak) -> Self {
        SampleChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for SampleChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        SampleChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for SampleChild {
    fn from(s: &'static str) -> Self {
        SampleChild::Text(s.into())
    }
}
impl From<String> for SampleChild {
    fn from(s: String) -> Self {
        SampleChild::Text(s.into())
    }
}
impl From<CowStr> for SampleChild {
    fn from(s: CowStr) -> Self {
        SampleChild::Text(s)
    }
}
impl std::fmt::Debug for SampleChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Button(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Code(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Data(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Image(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Input(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Label(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Link(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Map(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Object(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Output(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Select(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Small(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Span(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Template(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Time(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Video(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            SampleChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for SampleChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Audio(child) => std::fmt::Display::fmt(child, f),
            SampleChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            SampleChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Bold(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Button(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Cite(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Code(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Custom(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Data(child) => std::fmt::Display::fmt(child, f),
            SampleChild::DataList(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Definition(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Embed(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Image(child) => std::fmt::Display::fmt(child, f),
            SampleChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Input(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Italic(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Label(child) => std::fmt::Display::fmt(child, f),
            SampleChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Link(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Map(child) => std::fmt::Display::fmt(child, f),
            SampleChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Mark(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Meter(child) => std::fmt::Display::fmt(child, f),
            SampleChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Object(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Output(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Picture(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Progress(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Quote(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Sample(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Script(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Select(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Slot(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Small(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Span(child) => std::fmt::Display::fmt(child, f),
            SampleChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Strong(child) => std::fmt::Display::fmt(child, f),
            SampleChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            SampleChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Template(child) => std::fmt::Display::fmt(child, f),
            SampleChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Time(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Underline(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Variable(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Video(child) => std::fmt::Display::fmt(child, f),
            SampleChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            SampleChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
