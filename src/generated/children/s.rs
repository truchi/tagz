// 🤖 This file is generated!

use crate::*;
/// The `<s>` element's children.
#[derive(Clone)]
pub enum StrikeThroughChild {
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
impl From<Abbreviation> for StrikeThroughChild {
    fn from(child: Abbreviation) -> Self {
        StrikeThroughChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for StrikeThroughChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        StrikeThroughChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for StrikeThroughChild {
    fn from(child: Anchor) -> Self {
        StrikeThroughChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for StrikeThroughChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        StrikeThroughChild::Anchor(builder.build())
    }
}
impl From<Audio> for StrikeThroughChild {
    fn from(child: Audio) -> Self {
        StrikeThroughChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for StrikeThroughChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        StrikeThroughChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for StrikeThroughChild {
    fn from(child: BidirectionalIsolate) -> Self {
        StrikeThroughChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for StrikeThroughChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        StrikeThroughChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for StrikeThroughChild {
    fn from(child: BidirectionalOverride) -> Self {
        StrikeThroughChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for StrikeThroughChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        StrikeThroughChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for StrikeThroughChild {
    fn from(child: Bold) -> Self {
        StrikeThroughChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for StrikeThroughChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        StrikeThroughChild::Bold(builder.build())
    }
}
impl From<Button> for StrikeThroughChild {
    fn from(child: Button) -> Self {
        StrikeThroughChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for StrikeThroughChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        StrikeThroughChild::Button(builder.build())
    }
}
impl From<Canvas> for StrikeThroughChild {
    fn from(child: Canvas) -> Self {
        StrikeThroughChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for StrikeThroughChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        StrikeThroughChild::Canvas(builder.build())
    }
}
impl From<Cite> for StrikeThroughChild {
    fn from(child: Cite) -> Self {
        StrikeThroughChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for StrikeThroughChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        StrikeThroughChild::Cite(builder.build())
    }
}
impl From<Code> for StrikeThroughChild {
    fn from(child: Code) -> Self {
        StrikeThroughChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for StrikeThroughChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        StrikeThroughChild::Code(builder.build())
    }
}
impl From<Custom> for StrikeThroughChild {
    fn from(child: Custom) -> Self {
        StrikeThroughChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for StrikeThroughChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        StrikeThroughChild::Custom(builder.build())
    }
}
impl From<Data> for StrikeThroughChild {
    fn from(child: Data) -> Self {
        StrikeThroughChild::Data(child)
    }
}
impl From<builders::DataBuilder> for StrikeThroughChild {
    fn from(builder: builders::DataBuilder) -> Self {
        StrikeThroughChild::Data(builder.build())
    }
}
impl From<DataList> for StrikeThroughChild {
    fn from(child: DataList) -> Self {
        StrikeThroughChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for StrikeThroughChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        StrikeThroughChild::DataList(builder.build())
    }
}
impl From<Definition> for StrikeThroughChild {
    fn from(child: Definition) -> Self {
        StrikeThroughChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for StrikeThroughChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        StrikeThroughChild::Definition(builder.build())
    }
}
impl From<Deleted> for StrikeThroughChild {
    fn from(child: Deleted) -> Self {
        StrikeThroughChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for StrikeThroughChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        StrikeThroughChild::Deleted(builder.build())
    }
}
impl From<Embed> for StrikeThroughChild {
    fn from(child: Embed) -> Self {
        StrikeThroughChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for StrikeThroughChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        StrikeThroughChild::Embed(builder.build())
    }
}
impl From<Emphasis> for StrikeThroughChild {
    fn from(child: Emphasis) -> Self {
        StrikeThroughChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for StrikeThroughChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        StrikeThroughChild::Emphasis(builder.build())
    }
}
impl From<Image> for StrikeThroughChild {
    fn from(child: Image) -> Self {
        StrikeThroughChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for StrikeThroughChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        StrikeThroughChild::Image(builder.build())
    }
}
impl From<InlineFrame> for StrikeThroughChild {
    fn from(child: InlineFrame) -> Self {
        StrikeThroughChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for StrikeThroughChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        StrikeThroughChild::InlineFrame(builder.build())
    }
}
impl From<Input> for StrikeThroughChild {
    fn from(child: Input) -> Self {
        StrikeThroughChild::Input(child)
    }
}
impl From<builders::InputBuilder> for StrikeThroughChild {
    fn from(builder: builders::InputBuilder) -> Self {
        StrikeThroughChild::Input(builder.build())
    }
}
impl From<Inserted> for StrikeThroughChild {
    fn from(child: Inserted) -> Self {
        StrikeThroughChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for StrikeThroughChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        StrikeThroughChild::Inserted(builder.build())
    }
}
impl From<Italic> for StrikeThroughChild {
    fn from(child: Italic) -> Self {
        StrikeThroughChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for StrikeThroughChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        StrikeThroughChild::Italic(builder.build())
    }
}
impl From<Keyboard> for StrikeThroughChild {
    fn from(child: Keyboard) -> Self {
        StrikeThroughChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for StrikeThroughChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        StrikeThroughChild::Keyboard(builder.build())
    }
}
impl From<Label> for StrikeThroughChild {
    fn from(child: Label) -> Self {
        StrikeThroughChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for StrikeThroughChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        StrikeThroughChild::Label(builder.build())
    }
}
impl From<LineBreak> for StrikeThroughChild {
    fn from(child: LineBreak) -> Self {
        StrikeThroughChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for StrikeThroughChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        StrikeThroughChild::LineBreak(builder.build())
    }
}
impl From<Link> for StrikeThroughChild {
    fn from(child: Link) -> Self {
        StrikeThroughChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for StrikeThroughChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        StrikeThroughChild::Link(builder.build())
    }
}
impl From<Map> for StrikeThroughChild {
    fn from(child: Map) -> Self {
        StrikeThroughChild::Map(child)
    }
}
impl From<builders::MapBuilder> for StrikeThroughChild {
    fn from(builder: builders::MapBuilder) -> Self {
        StrikeThroughChild::Map(builder.build())
    }
}
impl From<MapArea> for StrikeThroughChild {
    fn from(child: MapArea) -> Self {
        StrikeThroughChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for StrikeThroughChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        StrikeThroughChild::MapArea(builder.build())
    }
}
impl From<Mark> for StrikeThroughChild {
    fn from(child: Mark) -> Self {
        StrikeThroughChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for StrikeThroughChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        StrikeThroughChild::Mark(builder.build())
    }
}
impl From<Metadata> for StrikeThroughChild {
    fn from(child: Metadata) -> Self {
        StrikeThroughChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for StrikeThroughChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        StrikeThroughChild::Metadata(builder.build())
    }
}
impl From<Meter> for StrikeThroughChild {
    fn from(child: Meter) -> Self {
        StrikeThroughChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for StrikeThroughChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        StrikeThroughChild::Meter(builder.build())
    }
}
impl From<NoScript> for StrikeThroughChild {
    fn from(child: NoScript) -> Self {
        StrikeThroughChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for StrikeThroughChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        StrikeThroughChild::NoScript(builder.build())
    }
}
impl From<Object> for StrikeThroughChild {
    fn from(child: Object) -> Self {
        StrikeThroughChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for StrikeThroughChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        StrikeThroughChild::Object(builder.build())
    }
}
impl From<Output> for StrikeThroughChild {
    fn from(child: Output) -> Self {
        StrikeThroughChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for StrikeThroughChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        StrikeThroughChild::Output(builder.build())
    }
}
impl From<Picture> for StrikeThroughChild {
    fn from(child: Picture) -> Self {
        StrikeThroughChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for StrikeThroughChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        StrikeThroughChild::Picture(builder.build())
    }
}
impl From<Progress> for StrikeThroughChild {
    fn from(child: Progress) -> Self {
        StrikeThroughChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for StrikeThroughChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        StrikeThroughChild::Progress(builder.build())
    }
}
impl From<Quote> for StrikeThroughChild {
    fn from(child: Quote) -> Self {
        StrikeThroughChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for StrikeThroughChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        StrikeThroughChild::Quote(builder.build())
    }
}
impl From<Ruby> for StrikeThroughChild {
    fn from(child: Ruby) -> Self {
        StrikeThroughChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for StrikeThroughChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        StrikeThroughChild::Ruby(builder.build())
    }
}
impl From<Sample> for StrikeThroughChild {
    fn from(child: Sample) -> Self {
        StrikeThroughChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for StrikeThroughChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        StrikeThroughChild::Sample(builder.build())
    }
}
impl From<Script> for StrikeThroughChild {
    fn from(child: Script) -> Self {
        StrikeThroughChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for StrikeThroughChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        StrikeThroughChild::Script(builder.build())
    }
}
impl From<Select> for StrikeThroughChild {
    fn from(child: Select) -> Self {
        StrikeThroughChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for StrikeThroughChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        StrikeThroughChild::Select(builder.build())
    }
}
impl From<Slot> for StrikeThroughChild {
    fn from(child: Slot) -> Self {
        StrikeThroughChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for StrikeThroughChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        StrikeThroughChild::Slot(builder.build())
    }
}
impl From<Small> for StrikeThroughChild {
    fn from(child: Small) -> Self {
        StrikeThroughChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for StrikeThroughChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        StrikeThroughChild::Small(builder.build())
    }
}
impl From<Span> for StrikeThroughChild {
    fn from(child: Span) -> Self {
        StrikeThroughChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for StrikeThroughChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        StrikeThroughChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for StrikeThroughChild {
    fn from(child: StrikeThrough) -> Self {
        StrikeThroughChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for StrikeThroughChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        StrikeThroughChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for StrikeThroughChild {
    fn from(child: Strong) -> Self {
        StrikeThroughChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for StrikeThroughChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        StrikeThroughChild::Strong(builder.build())
    }
}
impl From<SubScript> for StrikeThroughChild {
    fn from(child: SubScript) -> Self {
        StrikeThroughChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for StrikeThroughChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        StrikeThroughChild::SubScript(builder.build())
    }
}
impl From<SupScript> for StrikeThroughChild {
    fn from(child: SupScript) -> Self {
        StrikeThroughChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for StrikeThroughChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        StrikeThroughChild::SupScript(builder.build())
    }
}
impl From<Template> for StrikeThroughChild {
    fn from(child: Template) -> Self {
        StrikeThroughChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for StrikeThroughChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        StrikeThroughChild::Template(builder.build())
    }
}
impl From<TextArea> for StrikeThroughChild {
    fn from(child: TextArea) -> Self {
        StrikeThroughChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for StrikeThroughChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        StrikeThroughChild::TextArea(builder.build())
    }
}
impl From<Time> for StrikeThroughChild {
    fn from(child: Time) -> Self {
        StrikeThroughChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for StrikeThroughChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        StrikeThroughChild::Time(builder.build())
    }
}
impl From<Underline> for StrikeThroughChild {
    fn from(child: Underline) -> Self {
        StrikeThroughChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for StrikeThroughChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        StrikeThroughChild::Underline(builder.build())
    }
}
impl From<Variable> for StrikeThroughChild {
    fn from(child: Variable) -> Self {
        StrikeThroughChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for StrikeThroughChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        StrikeThroughChild::Variable(builder.build())
    }
}
impl From<Video> for StrikeThroughChild {
    fn from(child: Video) -> Self {
        StrikeThroughChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for StrikeThroughChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        StrikeThroughChild::Video(builder.build())
    }
}
impl From<WordBreak> for StrikeThroughChild {
    fn from(child: WordBreak) -> Self {
        StrikeThroughChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for StrikeThroughChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        StrikeThroughChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for StrikeThroughChild {
    fn from(s: &'static str) -> Self {
        StrikeThroughChild::Text(s.into())
    }
}
impl From<String> for StrikeThroughChild {
    fn from(s: String) -> Self {
        StrikeThroughChild::Text(s.into())
    }
}
impl From<CowStr> for StrikeThroughChild {
    fn from(s: CowStr) -> Self {
        StrikeThroughChild::Text(s)
    }
}
impl std::fmt::Debug for StrikeThroughChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrikeThroughChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            StrikeThroughChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            StrikeThroughChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Button(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Code(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Data(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Image(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Input(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Label(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Link(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Map(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Object(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Output(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Script(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Select(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Small(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Span(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Template(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Time(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Video(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            StrikeThroughChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for StrikeThroughChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrikeThroughChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Audio(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            StrikeThroughChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            StrikeThroughChild::Bold(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Button(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Cite(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Code(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Custom(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Data(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::DataList(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Definition(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Embed(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Image(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Input(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Italic(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Label(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Link(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Map(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Mark(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Meter(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Object(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Output(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Picture(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Progress(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Quote(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Sample(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Script(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Select(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Slot(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Small(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Span(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Strong(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Template(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Time(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Underline(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Variable(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Video(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            StrikeThroughChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
