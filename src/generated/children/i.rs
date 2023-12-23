// 🤖 This file is generated!

use crate::*;
/// The `<i>` element's children.
#[derive(Clone)]
pub enum ItalicChild {
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
impl From<Abbreviation> for ItalicChild {
    fn from(child: Abbreviation) -> Self {
        ItalicChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for ItalicChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        ItalicChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for ItalicChild {
    fn from(child: Anchor) -> Self {
        ItalicChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for ItalicChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        ItalicChild::Anchor(builder.build())
    }
}
impl From<Audio> for ItalicChild {
    fn from(child: Audio) -> Self {
        ItalicChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for ItalicChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        ItalicChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for ItalicChild {
    fn from(child: BidirectionalIsolate) -> Self {
        ItalicChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for ItalicChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        ItalicChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for ItalicChild {
    fn from(child: BidirectionalOverride) -> Self {
        ItalicChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for ItalicChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        ItalicChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for ItalicChild {
    fn from(child: Bold) -> Self {
        ItalicChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for ItalicChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        ItalicChild::Bold(builder.build())
    }
}
impl From<Button> for ItalicChild {
    fn from(child: Button) -> Self {
        ItalicChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for ItalicChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        ItalicChild::Button(builder.build())
    }
}
impl From<Canvas> for ItalicChild {
    fn from(child: Canvas) -> Self {
        ItalicChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for ItalicChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        ItalicChild::Canvas(builder.build())
    }
}
impl From<Cite> for ItalicChild {
    fn from(child: Cite) -> Self {
        ItalicChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for ItalicChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        ItalicChild::Cite(builder.build())
    }
}
impl From<Code> for ItalicChild {
    fn from(child: Code) -> Self {
        ItalicChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for ItalicChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        ItalicChild::Code(builder.build())
    }
}
impl From<Custom> for ItalicChild {
    fn from(child: Custom) -> Self {
        ItalicChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for ItalicChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        ItalicChild::Custom(builder.build())
    }
}
impl From<Data> for ItalicChild {
    fn from(child: Data) -> Self {
        ItalicChild::Data(child)
    }
}
impl From<builders::DataBuilder> for ItalicChild {
    fn from(builder: builders::DataBuilder) -> Self {
        ItalicChild::Data(builder.build())
    }
}
impl From<DataList> for ItalicChild {
    fn from(child: DataList) -> Self {
        ItalicChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for ItalicChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        ItalicChild::DataList(builder.build())
    }
}
impl From<Definition> for ItalicChild {
    fn from(child: Definition) -> Self {
        ItalicChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for ItalicChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        ItalicChild::Definition(builder.build())
    }
}
impl From<Deleted> for ItalicChild {
    fn from(child: Deleted) -> Self {
        ItalicChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for ItalicChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        ItalicChild::Deleted(builder.build())
    }
}
impl From<Embed> for ItalicChild {
    fn from(child: Embed) -> Self {
        ItalicChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for ItalicChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        ItalicChild::Embed(builder.build())
    }
}
impl From<Emphasis> for ItalicChild {
    fn from(child: Emphasis) -> Self {
        ItalicChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for ItalicChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        ItalicChild::Emphasis(builder.build())
    }
}
impl From<Image> for ItalicChild {
    fn from(child: Image) -> Self {
        ItalicChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for ItalicChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        ItalicChild::Image(builder.build())
    }
}
impl From<InlineFrame> for ItalicChild {
    fn from(child: InlineFrame) -> Self {
        ItalicChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for ItalicChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        ItalicChild::InlineFrame(builder.build())
    }
}
impl From<Input> for ItalicChild {
    fn from(child: Input) -> Self {
        ItalicChild::Input(child)
    }
}
impl From<builders::InputBuilder> for ItalicChild {
    fn from(builder: builders::InputBuilder) -> Self {
        ItalicChild::Input(builder.build())
    }
}
impl From<Inserted> for ItalicChild {
    fn from(child: Inserted) -> Self {
        ItalicChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for ItalicChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        ItalicChild::Inserted(builder.build())
    }
}
impl From<Italic> for ItalicChild {
    fn from(child: Italic) -> Self {
        ItalicChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for ItalicChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        ItalicChild::Italic(builder.build())
    }
}
impl From<Keyboard> for ItalicChild {
    fn from(child: Keyboard) -> Self {
        ItalicChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for ItalicChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        ItalicChild::Keyboard(builder.build())
    }
}
impl From<Label> for ItalicChild {
    fn from(child: Label) -> Self {
        ItalicChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for ItalicChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        ItalicChild::Label(builder.build())
    }
}
impl From<LineBreak> for ItalicChild {
    fn from(child: LineBreak) -> Self {
        ItalicChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for ItalicChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        ItalicChild::LineBreak(builder.build())
    }
}
impl From<Link> for ItalicChild {
    fn from(child: Link) -> Self {
        ItalicChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for ItalicChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        ItalicChild::Link(builder.build())
    }
}
impl From<Map> for ItalicChild {
    fn from(child: Map) -> Self {
        ItalicChild::Map(child)
    }
}
impl From<builders::MapBuilder> for ItalicChild {
    fn from(builder: builders::MapBuilder) -> Self {
        ItalicChild::Map(builder.build())
    }
}
impl From<MapArea> for ItalicChild {
    fn from(child: MapArea) -> Self {
        ItalicChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for ItalicChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        ItalicChild::MapArea(builder.build())
    }
}
impl From<Mark> for ItalicChild {
    fn from(child: Mark) -> Self {
        ItalicChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for ItalicChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        ItalicChild::Mark(builder.build())
    }
}
impl From<Metadata> for ItalicChild {
    fn from(child: Metadata) -> Self {
        ItalicChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for ItalicChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        ItalicChild::Metadata(builder.build())
    }
}
impl From<Meter> for ItalicChild {
    fn from(child: Meter) -> Self {
        ItalicChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for ItalicChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        ItalicChild::Meter(builder.build())
    }
}
impl From<NoScript> for ItalicChild {
    fn from(child: NoScript) -> Self {
        ItalicChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for ItalicChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        ItalicChild::NoScript(builder.build())
    }
}
impl From<Object> for ItalicChild {
    fn from(child: Object) -> Self {
        ItalicChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for ItalicChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        ItalicChild::Object(builder.build())
    }
}
impl From<Output> for ItalicChild {
    fn from(child: Output) -> Self {
        ItalicChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for ItalicChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        ItalicChild::Output(builder.build())
    }
}
impl From<Picture> for ItalicChild {
    fn from(child: Picture) -> Self {
        ItalicChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for ItalicChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        ItalicChild::Picture(builder.build())
    }
}
impl From<Progress> for ItalicChild {
    fn from(child: Progress) -> Self {
        ItalicChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for ItalicChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        ItalicChild::Progress(builder.build())
    }
}
impl From<Quote> for ItalicChild {
    fn from(child: Quote) -> Self {
        ItalicChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for ItalicChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        ItalicChild::Quote(builder.build())
    }
}
impl From<Ruby> for ItalicChild {
    fn from(child: Ruby) -> Self {
        ItalicChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for ItalicChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        ItalicChild::Ruby(builder.build())
    }
}
impl From<Sample> for ItalicChild {
    fn from(child: Sample) -> Self {
        ItalicChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for ItalicChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        ItalicChild::Sample(builder.build())
    }
}
impl From<Script> for ItalicChild {
    fn from(child: Script) -> Self {
        ItalicChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for ItalicChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        ItalicChild::Script(builder.build())
    }
}
impl From<Select> for ItalicChild {
    fn from(child: Select) -> Self {
        ItalicChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for ItalicChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        ItalicChild::Select(builder.build())
    }
}
impl From<Slot> for ItalicChild {
    fn from(child: Slot) -> Self {
        ItalicChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for ItalicChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        ItalicChild::Slot(builder.build())
    }
}
impl From<Small> for ItalicChild {
    fn from(child: Small) -> Self {
        ItalicChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for ItalicChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        ItalicChild::Small(builder.build())
    }
}
impl From<Span> for ItalicChild {
    fn from(child: Span) -> Self {
        ItalicChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for ItalicChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        ItalicChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for ItalicChild {
    fn from(child: StrikeThrough) -> Self {
        ItalicChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for ItalicChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        ItalicChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for ItalicChild {
    fn from(child: Strong) -> Self {
        ItalicChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for ItalicChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        ItalicChild::Strong(builder.build())
    }
}
impl From<SubScript> for ItalicChild {
    fn from(child: SubScript) -> Self {
        ItalicChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for ItalicChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        ItalicChild::SubScript(builder.build())
    }
}
impl From<SupScript> for ItalicChild {
    fn from(child: SupScript) -> Self {
        ItalicChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for ItalicChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        ItalicChild::SupScript(builder.build())
    }
}
impl From<Template> for ItalicChild {
    fn from(child: Template) -> Self {
        ItalicChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for ItalicChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        ItalicChild::Template(builder.build())
    }
}
impl From<TextArea> for ItalicChild {
    fn from(child: TextArea) -> Self {
        ItalicChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for ItalicChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        ItalicChild::TextArea(builder.build())
    }
}
impl From<Time> for ItalicChild {
    fn from(child: Time) -> Self {
        ItalicChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for ItalicChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        ItalicChild::Time(builder.build())
    }
}
impl From<Underline> for ItalicChild {
    fn from(child: Underline) -> Self {
        ItalicChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for ItalicChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        ItalicChild::Underline(builder.build())
    }
}
impl From<Variable> for ItalicChild {
    fn from(child: Variable) -> Self {
        ItalicChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for ItalicChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        ItalicChild::Variable(builder.build())
    }
}
impl From<Video> for ItalicChild {
    fn from(child: Video) -> Self {
        ItalicChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for ItalicChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        ItalicChild::Video(builder.build())
    }
}
impl From<WordBreak> for ItalicChild {
    fn from(child: WordBreak) -> Self {
        ItalicChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for ItalicChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        ItalicChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for ItalicChild {
    fn from(s: &'static str) -> Self {
        ItalicChild::Text(s.into())
    }
}
impl From<String> for ItalicChild {
    fn from(s: String) -> Self {
        ItalicChild::Text(s.into())
    }
}
impl From<CowStr> for ItalicChild {
    fn from(s: CowStr) -> Self {
        ItalicChild::Text(s)
    }
}
impl std::fmt::Debug for ItalicChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItalicChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Button(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Code(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Data(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Image(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Input(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Label(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Link(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Map(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Object(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Output(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Script(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Select(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Small(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Span(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Template(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Time(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Video(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            ItalicChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for ItalicChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItalicChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Audio(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Bold(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Button(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Cite(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Code(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Custom(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Data(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::DataList(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Definition(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Embed(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Image(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Input(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Italic(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Label(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Link(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Map(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Mark(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Meter(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Object(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Output(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Picture(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Progress(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Quote(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Sample(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Script(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Select(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Slot(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Small(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Span(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Strong(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Template(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Time(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Underline(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Variable(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Video(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            ItalicChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
