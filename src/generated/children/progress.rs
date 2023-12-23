// 🤖 This file is generated!

use crate::*;
/// The `<progress>` element's children.
#[derive(Clone)]
pub enum ProgressChild {
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
impl From<Abbreviation> for ProgressChild {
    fn from(child: Abbreviation) -> Self {
        ProgressChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for ProgressChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        ProgressChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for ProgressChild {
    fn from(child: Anchor) -> Self {
        ProgressChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for ProgressChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        ProgressChild::Anchor(builder.build())
    }
}
impl From<Audio> for ProgressChild {
    fn from(child: Audio) -> Self {
        ProgressChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for ProgressChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        ProgressChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for ProgressChild {
    fn from(child: BidirectionalIsolate) -> Self {
        ProgressChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for ProgressChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        ProgressChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for ProgressChild {
    fn from(child: BidirectionalOverride) -> Self {
        ProgressChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for ProgressChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        ProgressChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for ProgressChild {
    fn from(child: Bold) -> Self {
        ProgressChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for ProgressChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        ProgressChild::Bold(builder.build())
    }
}
impl From<Button> for ProgressChild {
    fn from(child: Button) -> Self {
        ProgressChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for ProgressChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        ProgressChild::Button(builder.build())
    }
}
impl From<Canvas> for ProgressChild {
    fn from(child: Canvas) -> Self {
        ProgressChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for ProgressChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        ProgressChild::Canvas(builder.build())
    }
}
impl From<Cite> for ProgressChild {
    fn from(child: Cite) -> Self {
        ProgressChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for ProgressChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        ProgressChild::Cite(builder.build())
    }
}
impl From<Code> for ProgressChild {
    fn from(child: Code) -> Self {
        ProgressChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for ProgressChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        ProgressChild::Code(builder.build())
    }
}
impl From<Custom> for ProgressChild {
    fn from(child: Custom) -> Self {
        ProgressChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for ProgressChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        ProgressChild::Custom(builder.build())
    }
}
impl From<Data> for ProgressChild {
    fn from(child: Data) -> Self {
        ProgressChild::Data(child)
    }
}
impl From<builders::DataBuilder> for ProgressChild {
    fn from(builder: builders::DataBuilder) -> Self {
        ProgressChild::Data(builder.build())
    }
}
impl From<DataList> for ProgressChild {
    fn from(child: DataList) -> Self {
        ProgressChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for ProgressChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        ProgressChild::DataList(builder.build())
    }
}
impl From<Definition> for ProgressChild {
    fn from(child: Definition) -> Self {
        ProgressChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for ProgressChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        ProgressChild::Definition(builder.build())
    }
}
impl From<Deleted> for ProgressChild {
    fn from(child: Deleted) -> Self {
        ProgressChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for ProgressChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        ProgressChild::Deleted(builder.build())
    }
}
impl From<Embed> for ProgressChild {
    fn from(child: Embed) -> Self {
        ProgressChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for ProgressChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        ProgressChild::Embed(builder.build())
    }
}
impl From<Emphasis> for ProgressChild {
    fn from(child: Emphasis) -> Self {
        ProgressChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for ProgressChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        ProgressChild::Emphasis(builder.build())
    }
}
impl From<Image> for ProgressChild {
    fn from(child: Image) -> Self {
        ProgressChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for ProgressChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        ProgressChild::Image(builder.build())
    }
}
impl From<InlineFrame> for ProgressChild {
    fn from(child: InlineFrame) -> Self {
        ProgressChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for ProgressChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        ProgressChild::InlineFrame(builder.build())
    }
}
impl From<Input> for ProgressChild {
    fn from(child: Input) -> Self {
        ProgressChild::Input(child)
    }
}
impl From<builders::InputBuilder> for ProgressChild {
    fn from(builder: builders::InputBuilder) -> Self {
        ProgressChild::Input(builder.build())
    }
}
impl From<Inserted> for ProgressChild {
    fn from(child: Inserted) -> Self {
        ProgressChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for ProgressChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        ProgressChild::Inserted(builder.build())
    }
}
impl From<Italic> for ProgressChild {
    fn from(child: Italic) -> Self {
        ProgressChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for ProgressChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        ProgressChild::Italic(builder.build())
    }
}
impl From<Keyboard> for ProgressChild {
    fn from(child: Keyboard) -> Self {
        ProgressChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for ProgressChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        ProgressChild::Keyboard(builder.build())
    }
}
impl From<Label> for ProgressChild {
    fn from(child: Label) -> Self {
        ProgressChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for ProgressChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        ProgressChild::Label(builder.build())
    }
}
impl From<LineBreak> for ProgressChild {
    fn from(child: LineBreak) -> Self {
        ProgressChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for ProgressChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        ProgressChild::LineBreak(builder.build())
    }
}
impl From<Link> for ProgressChild {
    fn from(child: Link) -> Self {
        ProgressChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for ProgressChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        ProgressChild::Link(builder.build())
    }
}
impl From<Map> for ProgressChild {
    fn from(child: Map) -> Self {
        ProgressChild::Map(child)
    }
}
impl From<builders::MapBuilder> for ProgressChild {
    fn from(builder: builders::MapBuilder) -> Self {
        ProgressChild::Map(builder.build())
    }
}
impl From<MapArea> for ProgressChild {
    fn from(child: MapArea) -> Self {
        ProgressChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for ProgressChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        ProgressChild::MapArea(builder.build())
    }
}
impl From<Mark> for ProgressChild {
    fn from(child: Mark) -> Self {
        ProgressChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for ProgressChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        ProgressChild::Mark(builder.build())
    }
}
impl From<Metadata> for ProgressChild {
    fn from(child: Metadata) -> Self {
        ProgressChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for ProgressChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        ProgressChild::Metadata(builder.build())
    }
}
impl From<Meter> for ProgressChild {
    fn from(child: Meter) -> Self {
        ProgressChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for ProgressChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        ProgressChild::Meter(builder.build())
    }
}
impl From<NoScript> for ProgressChild {
    fn from(child: NoScript) -> Self {
        ProgressChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for ProgressChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        ProgressChild::NoScript(builder.build())
    }
}
impl From<Object> for ProgressChild {
    fn from(child: Object) -> Self {
        ProgressChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for ProgressChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        ProgressChild::Object(builder.build())
    }
}
impl From<Output> for ProgressChild {
    fn from(child: Output) -> Self {
        ProgressChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for ProgressChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        ProgressChild::Output(builder.build())
    }
}
impl From<Picture> for ProgressChild {
    fn from(child: Picture) -> Self {
        ProgressChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for ProgressChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        ProgressChild::Picture(builder.build())
    }
}
impl From<Progress> for ProgressChild {
    fn from(child: Progress) -> Self {
        ProgressChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for ProgressChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        ProgressChild::Progress(builder.build())
    }
}
impl From<Quote> for ProgressChild {
    fn from(child: Quote) -> Self {
        ProgressChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for ProgressChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        ProgressChild::Quote(builder.build())
    }
}
impl From<Ruby> for ProgressChild {
    fn from(child: Ruby) -> Self {
        ProgressChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for ProgressChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        ProgressChild::Ruby(builder.build())
    }
}
impl From<Sample> for ProgressChild {
    fn from(child: Sample) -> Self {
        ProgressChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for ProgressChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        ProgressChild::Sample(builder.build())
    }
}
impl From<Script> for ProgressChild {
    fn from(child: Script) -> Self {
        ProgressChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for ProgressChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        ProgressChild::Script(builder.build())
    }
}
impl From<Select> for ProgressChild {
    fn from(child: Select) -> Self {
        ProgressChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for ProgressChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        ProgressChild::Select(builder.build())
    }
}
impl From<Slot> for ProgressChild {
    fn from(child: Slot) -> Self {
        ProgressChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for ProgressChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        ProgressChild::Slot(builder.build())
    }
}
impl From<Small> for ProgressChild {
    fn from(child: Small) -> Self {
        ProgressChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for ProgressChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        ProgressChild::Small(builder.build())
    }
}
impl From<Span> for ProgressChild {
    fn from(child: Span) -> Self {
        ProgressChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for ProgressChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        ProgressChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for ProgressChild {
    fn from(child: StrikeThrough) -> Self {
        ProgressChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for ProgressChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        ProgressChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for ProgressChild {
    fn from(child: Strong) -> Self {
        ProgressChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for ProgressChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        ProgressChild::Strong(builder.build())
    }
}
impl From<SubScript> for ProgressChild {
    fn from(child: SubScript) -> Self {
        ProgressChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for ProgressChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        ProgressChild::SubScript(builder.build())
    }
}
impl From<SupScript> for ProgressChild {
    fn from(child: SupScript) -> Self {
        ProgressChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for ProgressChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        ProgressChild::SupScript(builder.build())
    }
}
impl From<Template> for ProgressChild {
    fn from(child: Template) -> Self {
        ProgressChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for ProgressChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        ProgressChild::Template(builder.build())
    }
}
impl From<TextArea> for ProgressChild {
    fn from(child: TextArea) -> Self {
        ProgressChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for ProgressChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        ProgressChild::TextArea(builder.build())
    }
}
impl From<Time> for ProgressChild {
    fn from(child: Time) -> Self {
        ProgressChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for ProgressChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        ProgressChild::Time(builder.build())
    }
}
impl From<Underline> for ProgressChild {
    fn from(child: Underline) -> Self {
        ProgressChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for ProgressChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        ProgressChild::Underline(builder.build())
    }
}
impl From<Variable> for ProgressChild {
    fn from(child: Variable) -> Self {
        ProgressChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for ProgressChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        ProgressChild::Variable(builder.build())
    }
}
impl From<Video> for ProgressChild {
    fn from(child: Video) -> Self {
        ProgressChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for ProgressChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        ProgressChild::Video(builder.build())
    }
}
impl From<WordBreak> for ProgressChild {
    fn from(child: WordBreak) -> Self {
        ProgressChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for ProgressChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        ProgressChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for ProgressChild {
    fn from(s: &'static str) -> Self {
        ProgressChild::Text(s.into())
    }
}
impl From<String> for ProgressChild {
    fn from(s: String) -> Self {
        ProgressChild::Text(s.into())
    }
}
impl From<CowStr> for ProgressChild {
    fn from(s: CowStr) -> Self {
        ProgressChild::Text(s)
    }
}
impl std::fmt::Debug for ProgressChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Button(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Code(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Data(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Image(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Input(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Label(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Link(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Map(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Object(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Output(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Script(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Select(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Small(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Span(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Template(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Time(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Video(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            ProgressChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for ProgressChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Audio(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            ProgressChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            ProgressChild::Bold(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Button(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Cite(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Code(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Custom(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Data(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::DataList(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Definition(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Embed(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Image(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Input(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Italic(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Label(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Link(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Map(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Mark(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Meter(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Object(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Output(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Picture(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Progress(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Quote(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Sample(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Script(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Select(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Slot(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Small(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Span(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Strong(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Template(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Time(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Underline(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Variable(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Video(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            ProgressChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
