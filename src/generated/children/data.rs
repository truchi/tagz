// 🤖 This file is generated!

use crate::*;
/// The `<data>` element's children.
#[derive(Clone)]
pub enum DataChild {
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
impl From<Abbreviation> for DataChild {
    fn from(child: Abbreviation) -> Self {
        DataChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for DataChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        DataChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for DataChild {
    fn from(child: Anchor) -> Self {
        DataChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for DataChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        DataChild::Anchor(builder.build())
    }
}
impl From<Audio> for DataChild {
    fn from(child: Audio) -> Self {
        DataChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for DataChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        DataChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for DataChild {
    fn from(child: BidirectionalIsolate) -> Self {
        DataChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for DataChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        DataChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for DataChild {
    fn from(child: BidirectionalOverride) -> Self {
        DataChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for DataChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        DataChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for DataChild {
    fn from(child: Bold) -> Self {
        DataChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for DataChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        DataChild::Bold(builder.build())
    }
}
impl From<Button> for DataChild {
    fn from(child: Button) -> Self {
        DataChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for DataChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        DataChild::Button(builder.build())
    }
}
impl From<Canvas> for DataChild {
    fn from(child: Canvas) -> Self {
        DataChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for DataChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        DataChild::Canvas(builder.build())
    }
}
impl From<Cite> for DataChild {
    fn from(child: Cite) -> Self {
        DataChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for DataChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        DataChild::Cite(builder.build())
    }
}
impl From<Code> for DataChild {
    fn from(child: Code) -> Self {
        DataChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for DataChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        DataChild::Code(builder.build())
    }
}
impl From<Custom> for DataChild {
    fn from(child: Custom) -> Self {
        DataChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for DataChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        DataChild::Custom(builder.build())
    }
}
impl From<Data> for DataChild {
    fn from(child: Data) -> Self {
        DataChild::Data(child)
    }
}
impl From<builders::DataBuilder> for DataChild {
    fn from(builder: builders::DataBuilder) -> Self {
        DataChild::Data(builder.build())
    }
}
impl From<DataList> for DataChild {
    fn from(child: DataList) -> Self {
        DataChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for DataChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        DataChild::DataList(builder.build())
    }
}
impl From<Definition> for DataChild {
    fn from(child: Definition) -> Self {
        DataChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for DataChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        DataChild::Definition(builder.build())
    }
}
impl From<Deleted> for DataChild {
    fn from(child: Deleted) -> Self {
        DataChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for DataChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        DataChild::Deleted(builder.build())
    }
}
impl From<Embed> for DataChild {
    fn from(child: Embed) -> Self {
        DataChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for DataChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        DataChild::Embed(builder.build())
    }
}
impl From<Emphasis> for DataChild {
    fn from(child: Emphasis) -> Self {
        DataChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for DataChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        DataChild::Emphasis(builder.build())
    }
}
impl From<Image> for DataChild {
    fn from(child: Image) -> Self {
        DataChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for DataChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        DataChild::Image(builder.build())
    }
}
impl From<InlineFrame> for DataChild {
    fn from(child: InlineFrame) -> Self {
        DataChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for DataChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        DataChild::InlineFrame(builder.build())
    }
}
impl From<Input> for DataChild {
    fn from(child: Input) -> Self {
        DataChild::Input(child)
    }
}
impl From<builders::InputBuilder> for DataChild {
    fn from(builder: builders::InputBuilder) -> Self {
        DataChild::Input(builder.build())
    }
}
impl From<Inserted> for DataChild {
    fn from(child: Inserted) -> Self {
        DataChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for DataChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        DataChild::Inserted(builder.build())
    }
}
impl From<Italic> for DataChild {
    fn from(child: Italic) -> Self {
        DataChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for DataChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        DataChild::Italic(builder.build())
    }
}
impl From<Keyboard> for DataChild {
    fn from(child: Keyboard) -> Self {
        DataChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for DataChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        DataChild::Keyboard(builder.build())
    }
}
impl From<Label> for DataChild {
    fn from(child: Label) -> Self {
        DataChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for DataChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        DataChild::Label(builder.build())
    }
}
impl From<LineBreak> for DataChild {
    fn from(child: LineBreak) -> Self {
        DataChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for DataChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        DataChild::LineBreak(builder.build())
    }
}
impl From<Link> for DataChild {
    fn from(child: Link) -> Self {
        DataChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for DataChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        DataChild::Link(builder.build())
    }
}
impl From<Map> for DataChild {
    fn from(child: Map) -> Self {
        DataChild::Map(child)
    }
}
impl From<builders::MapBuilder> for DataChild {
    fn from(builder: builders::MapBuilder) -> Self {
        DataChild::Map(builder.build())
    }
}
impl From<MapArea> for DataChild {
    fn from(child: MapArea) -> Self {
        DataChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for DataChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        DataChild::MapArea(builder.build())
    }
}
impl From<Mark> for DataChild {
    fn from(child: Mark) -> Self {
        DataChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for DataChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        DataChild::Mark(builder.build())
    }
}
impl From<Metadata> for DataChild {
    fn from(child: Metadata) -> Self {
        DataChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for DataChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        DataChild::Metadata(builder.build())
    }
}
impl From<Meter> for DataChild {
    fn from(child: Meter) -> Self {
        DataChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for DataChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        DataChild::Meter(builder.build())
    }
}
impl From<NoScript> for DataChild {
    fn from(child: NoScript) -> Self {
        DataChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for DataChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        DataChild::NoScript(builder.build())
    }
}
impl From<Object> for DataChild {
    fn from(child: Object) -> Self {
        DataChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for DataChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        DataChild::Object(builder.build())
    }
}
impl From<Output> for DataChild {
    fn from(child: Output) -> Self {
        DataChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for DataChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        DataChild::Output(builder.build())
    }
}
impl From<Picture> for DataChild {
    fn from(child: Picture) -> Self {
        DataChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for DataChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        DataChild::Picture(builder.build())
    }
}
impl From<Progress> for DataChild {
    fn from(child: Progress) -> Self {
        DataChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for DataChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        DataChild::Progress(builder.build())
    }
}
impl From<Quote> for DataChild {
    fn from(child: Quote) -> Self {
        DataChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for DataChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        DataChild::Quote(builder.build())
    }
}
impl From<Ruby> for DataChild {
    fn from(child: Ruby) -> Self {
        DataChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for DataChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        DataChild::Ruby(builder.build())
    }
}
impl From<Sample> for DataChild {
    fn from(child: Sample) -> Self {
        DataChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for DataChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        DataChild::Sample(builder.build())
    }
}
impl From<Script> for DataChild {
    fn from(child: Script) -> Self {
        DataChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DataChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DataChild::Script(builder.build())
    }
}
impl From<Select> for DataChild {
    fn from(child: Select) -> Self {
        DataChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for DataChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        DataChild::Select(builder.build())
    }
}
impl From<Slot> for DataChild {
    fn from(child: Slot) -> Self {
        DataChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for DataChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        DataChild::Slot(builder.build())
    }
}
impl From<Small> for DataChild {
    fn from(child: Small) -> Self {
        DataChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for DataChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        DataChild::Small(builder.build())
    }
}
impl From<Span> for DataChild {
    fn from(child: Span) -> Self {
        DataChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for DataChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        DataChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for DataChild {
    fn from(child: StrikeThrough) -> Self {
        DataChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for DataChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        DataChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for DataChild {
    fn from(child: Strong) -> Self {
        DataChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for DataChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        DataChild::Strong(builder.build())
    }
}
impl From<SubScript> for DataChild {
    fn from(child: SubScript) -> Self {
        DataChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for DataChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        DataChild::SubScript(builder.build())
    }
}
impl From<SupScript> for DataChild {
    fn from(child: SupScript) -> Self {
        DataChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for DataChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        DataChild::SupScript(builder.build())
    }
}
impl From<Template> for DataChild {
    fn from(child: Template) -> Self {
        DataChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DataChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DataChild::Template(builder.build())
    }
}
impl From<TextArea> for DataChild {
    fn from(child: TextArea) -> Self {
        DataChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for DataChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        DataChild::TextArea(builder.build())
    }
}
impl From<Time> for DataChild {
    fn from(child: Time) -> Self {
        DataChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for DataChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        DataChild::Time(builder.build())
    }
}
impl From<Underline> for DataChild {
    fn from(child: Underline) -> Self {
        DataChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for DataChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        DataChild::Underline(builder.build())
    }
}
impl From<Variable> for DataChild {
    fn from(child: Variable) -> Self {
        DataChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for DataChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        DataChild::Variable(builder.build())
    }
}
impl From<Video> for DataChild {
    fn from(child: Video) -> Self {
        DataChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for DataChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        DataChild::Video(builder.build())
    }
}
impl From<WordBreak> for DataChild {
    fn from(child: WordBreak) -> Self {
        DataChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for DataChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        DataChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for DataChild {
    fn from(s: &'static str) -> Self {
        DataChild::Text(s.into())
    }
}
impl From<String> for DataChild {
    fn from(s: String) -> Self {
        DataChild::Text(s.into())
    }
}
impl From<CowStr> for DataChild {
    fn from(s: CowStr) -> Self {
        DataChild::Text(s)
    }
}
impl std::fmt::Debug for DataChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            DataChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            DataChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Button(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Code(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Data(child) => std::fmt::Debug::fmt(child, f),
            DataChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Image(child) => std::fmt::Debug::fmt(child, f),
            DataChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Input(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Label(child) => std::fmt::Debug::fmt(child, f),
            DataChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Link(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Map(child) => std::fmt::Debug::fmt(child, f),
            DataChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            DataChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Object(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Output(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Select(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Small(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Span(child) => std::fmt::Debug::fmt(child, f),
            DataChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            DataChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            DataChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Template(child) => std::fmt::Debug::fmt(child, f),
            DataChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Time(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Video(child) => std::fmt::Debug::fmt(child, f),
            DataChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            DataChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for DataChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            DataChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            DataChild::Audio(child) => std::fmt::Display::fmt(child, f),
            DataChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            DataChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            DataChild::Bold(child) => std::fmt::Display::fmt(child, f),
            DataChild::Button(child) => std::fmt::Display::fmt(child, f),
            DataChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            DataChild::Cite(child) => std::fmt::Display::fmt(child, f),
            DataChild::Code(child) => std::fmt::Display::fmt(child, f),
            DataChild::Custom(child) => std::fmt::Display::fmt(child, f),
            DataChild::Data(child) => std::fmt::Display::fmt(child, f),
            DataChild::DataList(child) => std::fmt::Display::fmt(child, f),
            DataChild::Definition(child) => std::fmt::Display::fmt(child, f),
            DataChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            DataChild::Embed(child) => std::fmt::Display::fmt(child, f),
            DataChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            DataChild::Image(child) => std::fmt::Display::fmt(child, f),
            DataChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            DataChild::Input(child) => std::fmt::Display::fmt(child, f),
            DataChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            DataChild::Italic(child) => std::fmt::Display::fmt(child, f),
            DataChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            DataChild::Label(child) => std::fmt::Display::fmt(child, f),
            DataChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            DataChild::Link(child) => std::fmt::Display::fmt(child, f),
            DataChild::Map(child) => std::fmt::Display::fmt(child, f),
            DataChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            DataChild::Mark(child) => std::fmt::Display::fmt(child, f),
            DataChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            DataChild::Meter(child) => std::fmt::Display::fmt(child, f),
            DataChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            DataChild::Object(child) => std::fmt::Display::fmt(child, f),
            DataChild::Output(child) => std::fmt::Display::fmt(child, f),
            DataChild::Picture(child) => std::fmt::Display::fmt(child, f),
            DataChild::Progress(child) => std::fmt::Display::fmt(child, f),
            DataChild::Quote(child) => std::fmt::Display::fmt(child, f),
            DataChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            DataChild::Sample(child) => std::fmt::Display::fmt(child, f),
            DataChild::Script(child) => std::fmt::Display::fmt(child, f),
            DataChild::Select(child) => std::fmt::Display::fmt(child, f),
            DataChild::Slot(child) => std::fmt::Display::fmt(child, f),
            DataChild::Small(child) => std::fmt::Display::fmt(child, f),
            DataChild::Span(child) => std::fmt::Display::fmt(child, f),
            DataChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            DataChild::Strong(child) => std::fmt::Display::fmt(child, f),
            DataChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            DataChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            DataChild::Template(child) => std::fmt::Display::fmt(child, f),
            DataChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            DataChild::Time(child) => std::fmt::Display::fmt(child, f),
            DataChild::Underline(child) => std::fmt::Display::fmt(child, f),
            DataChild::Variable(child) => std::fmt::Display::fmt(child, f),
            DataChild::Video(child) => std::fmt::Display::fmt(child, f),
            DataChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            DataChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
