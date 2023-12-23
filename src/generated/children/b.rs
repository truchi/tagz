// 🤖 This file is generated!

use crate::*;
/// The `<b>` element's children.
#[derive(Clone)]
pub enum BoldChild {
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
impl From<Abbreviation> for BoldChild {
    fn from(child: Abbreviation) -> Self {
        BoldChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for BoldChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        BoldChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for BoldChild {
    fn from(child: Anchor) -> Self {
        BoldChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for BoldChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        BoldChild::Anchor(builder.build())
    }
}
impl From<Audio> for BoldChild {
    fn from(child: Audio) -> Self {
        BoldChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for BoldChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        BoldChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for BoldChild {
    fn from(child: BidirectionalIsolate) -> Self {
        BoldChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for BoldChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        BoldChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for BoldChild {
    fn from(child: BidirectionalOverride) -> Self {
        BoldChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for BoldChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        BoldChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for BoldChild {
    fn from(child: Bold) -> Self {
        BoldChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for BoldChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        BoldChild::Bold(builder.build())
    }
}
impl From<Button> for BoldChild {
    fn from(child: Button) -> Self {
        BoldChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for BoldChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        BoldChild::Button(builder.build())
    }
}
impl From<Canvas> for BoldChild {
    fn from(child: Canvas) -> Self {
        BoldChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for BoldChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        BoldChild::Canvas(builder.build())
    }
}
impl From<Cite> for BoldChild {
    fn from(child: Cite) -> Self {
        BoldChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for BoldChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        BoldChild::Cite(builder.build())
    }
}
impl From<Code> for BoldChild {
    fn from(child: Code) -> Self {
        BoldChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for BoldChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        BoldChild::Code(builder.build())
    }
}
impl From<Custom> for BoldChild {
    fn from(child: Custom) -> Self {
        BoldChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for BoldChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        BoldChild::Custom(builder.build())
    }
}
impl From<Data> for BoldChild {
    fn from(child: Data) -> Self {
        BoldChild::Data(child)
    }
}
impl From<builders::DataBuilder> for BoldChild {
    fn from(builder: builders::DataBuilder) -> Self {
        BoldChild::Data(builder.build())
    }
}
impl From<DataList> for BoldChild {
    fn from(child: DataList) -> Self {
        BoldChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for BoldChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        BoldChild::DataList(builder.build())
    }
}
impl From<Definition> for BoldChild {
    fn from(child: Definition) -> Self {
        BoldChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for BoldChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        BoldChild::Definition(builder.build())
    }
}
impl From<Deleted> for BoldChild {
    fn from(child: Deleted) -> Self {
        BoldChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for BoldChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        BoldChild::Deleted(builder.build())
    }
}
impl From<Embed> for BoldChild {
    fn from(child: Embed) -> Self {
        BoldChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for BoldChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        BoldChild::Embed(builder.build())
    }
}
impl From<Emphasis> for BoldChild {
    fn from(child: Emphasis) -> Self {
        BoldChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for BoldChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        BoldChild::Emphasis(builder.build())
    }
}
impl From<Image> for BoldChild {
    fn from(child: Image) -> Self {
        BoldChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for BoldChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        BoldChild::Image(builder.build())
    }
}
impl From<InlineFrame> for BoldChild {
    fn from(child: InlineFrame) -> Self {
        BoldChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for BoldChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        BoldChild::InlineFrame(builder.build())
    }
}
impl From<Input> for BoldChild {
    fn from(child: Input) -> Self {
        BoldChild::Input(child)
    }
}
impl From<builders::InputBuilder> for BoldChild {
    fn from(builder: builders::InputBuilder) -> Self {
        BoldChild::Input(builder.build())
    }
}
impl From<Inserted> for BoldChild {
    fn from(child: Inserted) -> Self {
        BoldChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for BoldChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        BoldChild::Inserted(builder.build())
    }
}
impl From<Italic> for BoldChild {
    fn from(child: Italic) -> Self {
        BoldChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for BoldChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        BoldChild::Italic(builder.build())
    }
}
impl From<Keyboard> for BoldChild {
    fn from(child: Keyboard) -> Self {
        BoldChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for BoldChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        BoldChild::Keyboard(builder.build())
    }
}
impl From<Label> for BoldChild {
    fn from(child: Label) -> Self {
        BoldChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for BoldChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        BoldChild::Label(builder.build())
    }
}
impl From<LineBreak> for BoldChild {
    fn from(child: LineBreak) -> Self {
        BoldChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for BoldChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        BoldChild::LineBreak(builder.build())
    }
}
impl From<Link> for BoldChild {
    fn from(child: Link) -> Self {
        BoldChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for BoldChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        BoldChild::Link(builder.build())
    }
}
impl From<Map> for BoldChild {
    fn from(child: Map) -> Self {
        BoldChild::Map(child)
    }
}
impl From<builders::MapBuilder> for BoldChild {
    fn from(builder: builders::MapBuilder) -> Self {
        BoldChild::Map(builder.build())
    }
}
impl From<MapArea> for BoldChild {
    fn from(child: MapArea) -> Self {
        BoldChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for BoldChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        BoldChild::MapArea(builder.build())
    }
}
impl From<Mark> for BoldChild {
    fn from(child: Mark) -> Self {
        BoldChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for BoldChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        BoldChild::Mark(builder.build())
    }
}
impl From<Metadata> for BoldChild {
    fn from(child: Metadata) -> Self {
        BoldChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for BoldChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        BoldChild::Metadata(builder.build())
    }
}
impl From<Meter> for BoldChild {
    fn from(child: Meter) -> Self {
        BoldChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for BoldChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        BoldChild::Meter(builder.build())
    }
}
impl From<NoScript> for BoldChild {
    fn from(child: NoScript) -> Self {
        BoldChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for BoldChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        BoldChild::NoScript(builder.build())
    }
}
impl From<Object> for BoldChild {
    fn from(child: Object) -> Self {
        BoldChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for BoldChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        BoldChild::Object(builder.build())
    }
}
impl From<Output> for BoldChild {
    fn from(child: Output) -> Self {
        BoldChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for BoldChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        BoldChild::Output(builder.build())
    }
}
impl From<Picture> for BoldChild {
    fn from(child: Picture) -> Self {
        BoldChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for BoldChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        BoldChild::Picture(builder.build())
    }
}
impl From<Progress> for BoldChild {
    fn from(child: Progress) -> Self {
        BoldChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for BoldChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        BoldChild::Progress(builder.build())
    }
}
impl From<Quote> for BoldChild {
    fn from(child: Quote) -> Self {
        BoldChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for BoldChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        BoldChild::Quote(builder.build())
    }
}
impl From<Ruby> for BoldChild {
    fn from(child: Ruby) -> Self {
        BoldChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for BoldChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        BoldChild::Ruby(builder.build())
    }
}
impl From<Sample> for BoldChild {
    fn from(child: Sample) -> Self {
        BoldChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for BoldChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        BoldChild::Sample(builder.build())
    }
}
impl From<Script> for BoldChild {
    fn from(child: Script) -> Self {
        BoldChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for BoldChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        BoldChild::Script(builder.build())
    }
}
impl From<Select> for BoldChild {
    fn from(child: Select) -> Self {
        BoldChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for BoldChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        BoldChild::Select(builder.build())
    }
}
impl From<Slot> for BoldChild {
    fn from(child: Slot) -> Self {
        BoldChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for BoldChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        BoldChild::Slot(builder.build())
    }
}
impl From<Small> for BoldChild {
    fn from(child: Small) -> Self {
        BoldChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for BoldChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        BoldChild::Small(builder.build())
    }
}
impl From<Span> for BoldChild {
    fn from(child: Span) -> Self {
        BoldChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for BoldChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        BoldChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for BoldChild {
    fn from(child: StrikeThrough) -> Self {
        BoldChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for BoldChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        BoldChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for BoldChild {
    fn from(child: Strong) -> Self {
        BoldChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for BoldChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        BoldChild::Strong(builder.build())
    }
}
impl From<SubScript> for BoldChild {
    fn from(child: SubScript) -> Self {
        BoldChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for BoldChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        BoldChild::SubScript(builder.build())
    }
}
impl From<SupScript> for BoldChild {
    fn from(child: SupScript) -> Self {
        BoldChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for BoldChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        BoldChild::SupScript(builder.build())
    }
}
impl From<Template> for BoldChild {
    fn from(child: Template) -> Self {
        BoldChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for BoldChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        BoldChild::Template(builder.build())
    }
}
impl From<TextArea> for BoldChild {
    fn from(child: TextArea) -> Self {
        BoldChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for BoldChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        BoldChild::TextArea(builder.build())
    }
}
impl From<Time> for BoldChild {
    fn from(child: Time) -> Self {
        BoldChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for BoldChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        BoldChild::Time(builder.build())
    }
}
impl From<Underline> for BoldChild {
    fn from(child: Underline) -> Self {
        BoldChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for BoldChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        BoldChild::Underline(builder.build())
    }
}
impl From<Variable> for BoldChild {
    fn from(child: Variable) -> Self {
        BoldChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for BoldChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        BoldChild::Variable(builder.build())
    }
}
impl From<Video> for BoldChild {
    fn from(child: Video) -> Self {
        BoldChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for BoldChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        BoldChild::Video(builder.build())
    }
}
impl From<WordBreak> for BoldChild {
    fn from(child: WordBreak) -> Self {
        BoldChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for BoldChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        BoldChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for BoldChild {
    fn from(s: &'static str) -> Self {
        BoldChild::Text(s.into())
    }
}
impl From<String> for BoldChild {
    fn from(s: String) -> Self {
        BoldChild::Text(s.into())
    }
}
impl From<CowStr> for BoldChild {
    fn from(s: CowStr) -> Self {
        BoldChild::Text(s)
    }
}
impl std::fmt::Debug for BoldChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoldChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Button(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Code(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Data(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Image(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Input(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Label(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Link(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Map(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Object(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Output(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Script(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Select(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Small(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Span(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Template(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Time(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Video(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            BoldChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for BoldChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoldChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Audio(child) => std::fmt::Display::fmt(child, f),
            BoldChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            BoldChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Bold(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Button(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Cite(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Code(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Custom(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Data(child) => std::fmt::Display::fmt(child, f),
            BoldChild::DataList(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Definition(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Embed(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Image(child) => std::fmt::Display::fmt(child, f),
            BoldChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Input(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Italic(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Label(child) => std::fmt::Display::fmt(child, f),
            BoldChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Link(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Map(child) => std::fmt::Display::fmt(child, f),
            BoldChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Mark(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Meter(child) => std::fmt::Display::fmt(child, f),
            BoldChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Object(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Output(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Picture(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Progress(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Quote(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Sample(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Script(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Select(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Slot(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Small(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Span(child) => std::fmt::Display::fmt(child, f),
            BoldChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Strong(child) => std::fmt::Display::fmt(child, f),
            BoldChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            BoldChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Template(child) => std::fmt::Display::fmt(child, f),
            BoldChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Time(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Underline(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Variable(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Video(child) => std::fmt::Display::fmt(child, f),
            BoldChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            BoldChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
