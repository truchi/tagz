// 🤖 This file is generated!

use crate::*;
/// The `<var>` element's children.
#[derive(Clone)]
pub enum VariableChild {
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
impl From<Abbreviation> for VariableChild {
    fn from(child: Abbreviation) -> Self {
        VariableChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for VariableChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        VariableChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for VariableChild {
    fn from(child: Anchor) -> Self {
        VariableChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for VariableChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        VariableChild::Anchor(builder.build())
    }
}
impl From<Audio> for VariableChild {
    fn from(child: Audio) -> Self {
        VariableChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for VariableChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        VariableChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for VariableChild {
    fn from(child: BidirectionalIsolate) -> Self {
        VariableChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for VariableChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        VariableChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for VariableChild {
    fn from(child: BidirectionalOverride) -> Self {
        VariableChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for VariableChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        VariableChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for VariableChild {
    fn from(child: Bold) -> Self {
        VariableChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for VariableChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        VariableChild::Bold(builder.build())
    }
}
impl From<Button> for VariableChild {
    fn from(child: Button) -> Self {
        VariableChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for VariableChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        VariableChild::Button(builder.build())
    }
}
impl From<Canvas> for VariableChild {
    fn from(child: Canvas) -> Self {
        VariableChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for VariableChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        VariableChild::Canvas(builder.build())
    }
}
impl From<Cite> for VariableChild {
    fn from(child: Cite) -> Self {
        VariableChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for VariableChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        VariableChild::Cite(builder.build())
    }
}
impl From<Code> for VariableChild {
    fn from(child: Code) -> Self {
        VariableChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for VariableChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        VariableChild::Code(builder.build())
    }
}
impl From<Custom> for VariableChild {
    fn from(child: Custom) -> Self {
        VariableChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for VariableChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        VariableChild::Custom(builder.build())
    }
}
impl From<Data> for VariableChild {
    fn from(child: Data) -> Self {
        VariableChild::Data(child)
    }
}
impl From<builders::DataBuilder> for VariableChild {
    fn from(builder: builders::DataBuilder) -> Self {
        VariableChild::Data(builder.build())
    }
}
impl From<DataList> for VariableChild {
    fn from(child: DataList) -> Self {
        VariableChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for VariableChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        VariableChild::DataList(builder.build())
    }
}
impl From<Definition> for VariableChild {
    fn from(child: Definition) -> Self {
        VariableChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for VariableChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        VariableChild::Definition(builder.build())
    }
}
impl From<Deleted> for VariableChild {
    fn from(child: Deleted) -> Self {
        VariableChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for VariableChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        VariableChild::Deleted(builder.build())
    }
}
impl From<Embed> for VariableChild {
    fn from(child: Embed) -> Self {
        VariableChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for VariableChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        VariableChild::Embed(builder.build())
    }
}
impl From<Emphasis> for VariableChild {
    fn from(child: Emphasis) -> Self {
        VariableChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for VariableChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        VariableChild::Emphasis(builder.build())
    }
}
impl From<Image> for VariableChild {
    fn from(child: Image) -> Self {
        VariableChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for VariableChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        VariableChild::Image(builder.build())
    }
}
impl From<InlineFrame> for VariableChild {
    fn from(child: InlineFrame) -> Self {
        VariableChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for VariableChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        VariableChild::InlineFrame(builder.build())
    }
}
impl From<Input> for VariableChild {
    fn from(child: Input) -> Self {
        VariableChild::Input(child)
    }
}
impl From<builders::InputBuilder> for VariableChild {
    fn from(builder: builders::InputBuilder) -> Self {
        VariableChild::Input(builder.build())
    }
}
impl From<Inserted> for VariableChild {
    fn from(child: Inserted) -> Self {
        VariableChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for VariableChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        VariableChild::Inserted(builder.build())
    }
}
impl From<Italic> for VariableChild {
    fn from(child: Italic) -> Self {
        VariableChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for VariableChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        VariableChild::Italic(builder.build())
    }
}
impl From<Keyboard> for VariableChild {
    fn from(child: Keyboard) -> Self {
        VariableChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for VariableChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        VariableChild::Keyboard(builder.build())
    }
}
impl From<Label> for VariableChild {
    fn from(child: Label) -> Self {
        VariableChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for VariableChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        VariableChild::Label(builder.build())
    }
}
impl From<LineBreak> for VariableChild {
    fn from(child: LineBreak) -> Self {
        VariableChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for VariableChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        VariableChild::LineBreak(builder.build())
    }
}
impl From<Link> for VariableChild {
    fn from(child: Link) -> Self {
        VariableChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for VariableChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        VariableChild::Link(builder.build())
    }
}
impl From<Map> for VariableChild {
    fn from(child: Map) -> Self {
        VariableChild::Map(child)
    }
}
impl From<builders::MapBuilder> for VariableChild {
    fn from(builder: builders::MapBuilder) -> Self {
        VariableChild::Map(builder.build())
    }
}
impl From<MapArea> for VariableChild {
    fn from(child: MapArea) -> Self {
        VariableChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for VariableChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        VariableChild::MapArea(builder.build())
    }
}
impl From<Mark> for VariableChild {
    fn from(child: Mark) -> Self {
        VariableChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for VariableChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        VariableChild::Mark(builder.build())
    }
}
impl From<Metadata> for VariableChild {
    fn from(child: Metadata) -> Self {
        VariableChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for VariableChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        VariableChild::Metadata(builder.build())
    }
}
impl From<Meter> for VariableChild {
    fn from(child: Meter) -> Self {
        VariableChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for VariableChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        VariableChild::Meter(builder.build())
    }
}
impl From<NoScript> for VariableChild {
    fn from(child: NoScript) -> Self {
        VariableChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for VariableChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        VariableChild::NoScript(builder.build())
    }
}
impl From<Object> for VariableChild {
    fn from(child: Object) -> Self {
        VariableChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for VariableChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        VariableChild::Object(builder.build())
    }
}
impl From<Output> for VariableChild {
    fn from(child: Output) -> Self {
        VariableChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for VariableChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        VariableChild::Output(builder.build())
    }
}
impl From<Picture> for VariableChild {
    fn from(child: Picture) -> Self {
        VariableChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for VariableChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        VariableChild::Picture(builder.build())
    }
}
impl From<Progress> for VariableChild {
    fn from(child: Progress) -> Self {
        VariableChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for VariableChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        VariableChild::Progress(builder.build())
    }
}
impl From<Quote> for VariableChild {
    fn from(child: Quote) -> Self {
        VariableChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for VariableChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        VariableChild::Quote(builder.build())
    }
}
impl From<Ruby> for VariableChild {
    fn from(child: Ruby) -> Self {
        VariableChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for VariableChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        VariableChild::Ruby(builder.build())
    }
}
impl From<Sample> for VariableChild {
    fn from(child: Sample) -> Self {
        VariableChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for VariableChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        VariableChild::Sample(builder.build())
    }
}
impl From<Script> for VariableChild {
    fn from(child: Script) -> Self {
        VariableChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for VariableChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        VariableChild::Script(builder.build())
    }
}
impl From<Select> for VariableChild {
    fn from(child: Select) -> Self {
        VariableChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for VariableChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        VariableChild::Select(builder.build())
    }
}
impl From<Slot> for VariableChild {
    fn from(child: Slot) -> Self {
        VariableChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for VariableChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        VariableChild::Slot(builder.build())
    }
}
impl From<Small> for VariableChild {
    fn from(child: Small) -> Self {
        VariableChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for VariableChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        VariableChild::Small(builder.build())
    }
}
impl From<Span> for VariableChild {
    fn from(child: Span) -> Self {
        VariableChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for VariableChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        VariableChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for VariableChild {
    fn from(child: StrikeThrough) -> Self {
        VariableChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for VariableChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        VariableChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for VariableChild {
    fn from(child: Strong) -> Self {
        VariableChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for VariableChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        VariableChild::Strong(builder.build())
    }
}
impl From<SubScript> for VariableChild {
    fn from(child: SubScript) -> Self {
        VariableChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for VariableChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        VariableChild::SubScript(builder.build())
    }
}
impl From<SupScript> for VariableChild {
    fn from(child: SupScript) -> Self {
        VariableChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for VariableChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        VariableChild::SupScript(builder.build())
    }
}
impl From<Template> for VariableChild {
    fn from(child: Template) -> Self {
        VariableChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for VariableChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        VariableChild::Template(builder.build())
    }
}
impl From<TextArea> for VariableChild {
    fn from(child: TextArea) -> Self {
        VariableChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for VariableChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        VariableChild::TextArea(builder.build())
    }
}
impl From<Time> for VariableChild {
    fn from(child: Time) -> Self {
        VariableChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for VariableChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        VariableChild::Time(builder.build())
    }
}
impl From<Underline> for VariableChild {
    fn from(child: Underline) -> Self {
        VariableChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for VariableChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        VariableChild::Underline(builder.build())
    }
}
impl From<Variable> for VariableChild {
    fn from(child: Variable) -> Self {
        VariableChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for VariableChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        VariableChild::Variable(builder.build())
    }
}
impl From<Video> for VariableChild {
    fn from(child: Video) -> Self {
        VariableChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for VariableChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        VariableChild::Video(builder.build())
    }
}
impl From<WordBreak> for VariableChild {
    fn from(child: WordBreak) -> Self {
        VariableChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for VariableChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        VariableChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for VariableChild {
    fn from(s: &'static str) -> Self {
        VariableChild::Text(s.into())
    }
}
impl From<String> for VariableChild {
    fn from(s: String) -> Self {
        VariableChild::Text(s.into())
    }
}
impl From<CowStr> for VariableChild {
    fn from(s: CowStr) -> Self {
        VariableChild::Text(s)
    }
}
impl std::fmt::Debug for VariableChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Button(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Code(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Data(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Image(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Input(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Label(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Link(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Map(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Object(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Output(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Script(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Select(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Small(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Span(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Template(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Time(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Video(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            VariableChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for VariableChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Audio(child) => std::fmt::Display::fmt(child, f),
            VariableChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            VariableChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            VariableChild::Bold(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Button(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Cite(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Code(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Custom(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Data(child) => std::fmt::Display::fmt(child, f),
            VariableChild::DataList(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Definition(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Embed(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Image(child) => std::fmt::Display::fmt(child, f),
            VariableChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Input(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Italic(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Label(child) => std::fmt::Display::fmt(child, f),
            VariableChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Link(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Map(child) => std::fmt::Display::fmt(child, f),
            VariableChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Mark(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Meter(child) => std::fmt::Display::fmt(child, f),
            VariableChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Object(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Output(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Picture(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Progress(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Quote(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Sample(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Script(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Select(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Slot(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Small(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Span(child) => std::fmt::Display::fmt(child, f),
            VariableChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Strong(child) => std::fmt::Display::fmt(child, f),
            VariableChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            VariableChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Template(child) => std::fmt::Display::fmt(child, f),
            VariableChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Time(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Underline(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Variable(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Video(child) => std::fmt::Display::fmt(child, f),
            VariableChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            VariableChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
