// 🤖 This file is generated!

use crate::*;
/// The `<legend>` element's children.
#[derive(Clone)]
pub enum FieldSetLegendChild {
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
impl From<Abbreviation> for FieldSetLegendChild {
    fn from(child: Abbreviation) -> Self {
        FieldSetLegendChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for FieldSetLegendChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        FieldSetLegendChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for FieldSetLegendChild {
    fn from(child: Anchor) -> Self {
        FieldSetLegendChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for FieldSetLegendChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        FieldSetLegendChild::Anchor(builder.build())
    }
}
impl From<Audio> for FieldSetLegendChild {
    fn from(child: Audio) -> Self {
        FieldSetLegendChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for FieldSetLegendChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        FieldSetLegendChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for FieldSetLegendChild {
    fn from(child: BidirectionalIsolate) -> Self {
        FieldSetLegendChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for FieldSetLegendChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        FieldSetLegendChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for FieldSetLegendChild {
    fn from(child: BidirectionalOverride) -> Self {
        FieldSetLegendChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for FieldSetLegendChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        FieldSetLegendChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for FieldSetLegendChild {
    fn from(child: Bold) -> Self {
        FieldSetLegendChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for FieldSetLegendChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        FieldSetLegendChild::Bold(builder.build())
    }
}
impl From<Button> for FieldSetLegendChild {
    fn from(child: Button) -> Self {
        FieldSetLegendChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for FieldSetLegendChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        FieldSetLegendChild::Button(builder.build())
    }
}
impl From<Canvas> for FieldSetLegendChild {
    fn from(child: Canvas) -> Self {
        FieldSetLegendChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for FieldSetLegendChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        FieldSetLegendChild::Canvas(builder.build())
    }
}
impl From<Cite> for FieldSetLegendChild {
    fn from(child: Cite) -> Self {
        FieldSetLegendChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for FieldSetLegendChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        FieldSetLegendChild::Cite(builder.build())
    }
}
impl From<Code> for FieldSetLegendChild {
    fn from(child: Code) -> Self {
        FieldSetLegendChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for FieldSetLegendChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        FieldSetLegendChild::Code(builder.build())
    }
}
impl From<Custom> for FieldSetLegendChild {
    fn from(child: Custom) -> Self {
        FieldSetLegendChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for FieldSetLegendChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        FieldSetLegendChild::Custom(builder.build())
    }
}
impl From<Data> for FieldSetLegendChild {
    fn from(child: Data) -> Self {
        FieldSetLegendChild::Data(child)
    }
}
impl From<builders::DataBuilder> for FieldSetLegendChild {
    fn from(builder: builders::DataBuilder) -> Self {
        FieldSetLegendChild::Data(builder.build())
    }
}
impl From<DataList> for FieldSetLegendChild {
    fn from(child: DataList) -> Self {
        FieldSetLegendChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for FieldSetLegendChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        FieldSetLegendChild::DataList(builder.build())
    }
}
impl From<Definition> for FieldSetLegendChild {
    fn from(child: Definition) -> Self {
        FieldSetLegendChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for FieldSetLegendChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        FieldSetLegendChild::Definition(builder.build())
    }
}
impl From<Deleted> for FieldSetLegendChild {
    fn from(child: Deleted) -> Self {
        FieldSetLegendChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for FieldSetLegendChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        FieldSetLegendChild::Deleted(builder.build())
    }
}
impl From<Embed> for FieldSetLegendChild {
    fn from(child: Embed) -> Self {
        FieldSetLegendChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for FieldSetLegendChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        FieldSetLegendChild::Embed(builder.build())
    }
}
impl From<Emphasis> for FieldSetLegendChild {
    fn from(child: Emphasis) -> Self {
        FieldSetLegendChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for FieldSetLegendChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        FieldSetLegendChild::Emphasis(builder.build())
    }
}
impl From<Image> for FieldSetLegendChild {
    fn from(child: Image) -> Self {
        FieldSetLegendChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for FieldSetLegendChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        FieldSetLegendChild::Image(builder.build())
    }
}
impl From<InlineFrame> for FieldSetLegendChild {
    fn from(child: InlineFrame) -> Self {
        FieldSetLegendChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for FieldSetLegendChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        FieldSetLegendChild::InlineFrame(builder.build())
    }
}
impl From<Input> for FieldSetLegendChild {
    fn from(child: Input) -> Self {
        FieldSetLegendChild::Input(child)
    }
}
impl From<builders::InputBuilder> for FieldSetLegendChild {
    fn from(builder: builders::InputBuilder) -> Self {
        FieldSetLegendChild::Input(builder.build())
    }
}
impl From<Inserted> for FieldSetLegendChild {
    fn from(child: Inserted) -> Self {
        FieldSetLegendChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for FieldSetLegendChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        FieldSetLegendChild::Inserted(builder.build())
    }
}
impl From<Italic> for FieldSetLegendChild {
    fn from(child: Italic) -> Self {
        FieldSetLegendChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for FieldSetLegendChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        FieldSetLegendChild::Italic(builder.build())
    }
}
impl From<Keyboard> for FieldSetLegendChild {
    fn from(child: Keyboard) -> Self {
        FieldSetLegendChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for FieldSetLegendChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        FieldSetLegendChild::Keyboard(builder.build())
    }
}
impl From<Label> for FieldSetLegendChild {
    fn from(child: Label) -> Self {
        FieldSetLegendChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for FieldSetLegendChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        FieldSetLegendChild::Label(builder.build())
    }
}
impl From<LineBreak> for FieldSetLegendChild {
    fn from(child: LineBreak) -> Self {
        FieldSetLegendChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for FieldSetLegendChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        FieldSetLegendChild::LineBreak(builder.build())
    }
}
impl From<Link> for FieldSetLegendChild {
    fn from(child: Link) -> Self {
        FieldSetLegendChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for FieldSetLegendChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        FieldSetLegendChild::Link(builder.build())
    }
}
impl From<Map> for FieldSetLegendChild {
    fn from(child: Map) -> Self {
        FieldSetLegendChild::Map(child)
    }
}
impl From<builders::MapBuilder> for FieldSetLegendChild {
    fn from(builder: builders::MapBuilder) -> Self {
        FieldSetLegendChild::Map(builder.build())
    }
}
impl From<MapArea> for FieldSetLegendChild {
    fn from(child: MapArea) -> Self {
        FieldSetLegendChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for FieldSetLegendChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        FieldSetLegendChild::MapArea(builder.build())
    }
}
impl From<Mark> for FieldSetLegendChild {
    fn from(child: Mark) -> Self {
        FieldSetLegendChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for FieldSetLegendChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        FieldSetLegendChild::Mark(builder.build())
    }
}
impl From<Metadata> for FieldSetLegendChild {
    fn from(child: Metadata) -> Self {
        FieldSetLegendChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for FieldSetLegendChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        FieldSetLegendChild::Metadata(builder.build())
    }
}
impl From<Meter> for FieldSetLegendChild {
    fn from(child: Meter) -> Self {
        FieldSetLegendChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for FieldSetLegendChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        FieldSetLegendChild::Meter(builder.build())
    }
}
impl From<NoScript> for FieldSetLegendChild {
    fn from(child: NoScript) -> Self {
        FieldSetLegendChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for FieldSetLegendChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        FieldSetLegendChild::NoScript(builder.build())
    }
}
impl From<Object> for FieldSetLegendChild {
    fn from(child: Object) -> Self {
        FieldSetLegendChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for FieldSetLegendChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        FieldSetLegendChild::Object(builder.build())
    }
}
impl From<Output> for FieldSetLegendChild {
    fn from(child: Output) -> Self {
        FieldSetLegendChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for FieldSetLegendChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        FieldSetLegendChild::Output(builder.build())
    }
}
impl From<Picture> for FieldSetLegendChild {
    fn from(child: Picture) -> Self {
        FieldSetLegendChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for FieldSetLegendChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        FieldSetLegendChild::Picture(builder.build())
    }
}
impl From<Progress> for FieldSetLegendChild {
    fn from(child: Progress) -> Self {
        FieldSetLegendChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for FieldSetLegendChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        FieldSetLegendChild::Progress(builder.build())
    }
}
impl From<Quote> for FieldSetLegendChild {
    fn from(child: Quote) -> Self {
        FieldSetLegendChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for FieldSetLegendChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        FieldSetLegendChild::Quote(builder.build())
    }
}
impl From<Ruby> for FieldSetLegendChild {
    fn from(child: Ruby) -> Self {
        FieldSetLegendChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for FieldSetLegendChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        FieldSetLegendChild::Ruby(builder.build())
    }
}
impl From<Sample> for FieldSetLegendChild {
    fn from(child: Sample) -> Self {
        FieldSetLegendChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for FieldSetLegendChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        FieldSetLegendChild::Sample(builder.build())
    }
}
impl From<Script> for FieldSetLegendChild {
    fn from(child: Script) -> Self {
        FieldSetLegendChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for FieldSetLegendChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        FieldSetLegendChild::Script(builder.build())
    }
}
impl From<Select> for FieldSetLegendChild {
    fn from(child: Select) -> Self {
        FieldSetLegendChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for FieldSetLegendChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        FieldSetLegendChild::Select(builder.build())
    }
}
impl From<Slot> for FieldSetLegendChild {
    fn from(child: Slot) -> Self {
        FieldSetLegendChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for FieldSetLegendChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        FieldSetLegendChild::Slot(builder.build())
    }
}
impl From<Small> for FieldSetLegendChild {
    fn from(child: Small) -> Self {
        FieldSetLegendChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for FieldSetLegendChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        FieldSetLegendChild::Small(builder.build())
    }
}
impl From<Span> for FieldSetLegendChild {
    fn from(child: Span) -> Self {
        FieldSetLegendChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for FieldSetLegendChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        FieldSetLegendChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for FieldSetLegendChild {
    fn from(child: StrikeThrough) -> Self {
        FieldSetLegendChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for FieldSetLegendChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        FieldSetLegendChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for FieldSetLegendChild {
    fn from(child: Strong) -> Self {
        FieldSetLegendChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for FieldSetLegendChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        FieldSetLegendChild::Strong(builder.build())
    }
}
impl From<SubScript> for FieldSetLegendChild {
    fn from(child: SubScript) -> Self {
        FieldSetLegendChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for FieldSetLegendChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        FieldSetLegendChild::SubScript(builder.build())
    }
}
impl From<SupScript> for FieldSetLegendChild {
    fn from(child: SupScript) -> Self {
        FieldSetLegendChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for FieldSetLegendChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        FieldSetLegendChild::SupScript(builder.build())
    }
}
impl From<Template> for FieldSetLegendChild {
    fn from(child: Template) -> Self {
        FieldSetLegendChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for FieldSetLegendChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        FieldSetLegendChild::Template(builder.build())
    }
}
impl From<TextArea> for FieldSetLegendChild {
    fn from(child: TextArea) -> Self {
        FieldSetLegendChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for FieldSetLegendChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        FieldSetLegendChild::TextArea(builder.build())
    }
}
impl From<Time> for FieldSetLegendChild {
    fn from(child: Time) -> Self {
        FieldSetLegendChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for FieldSetLegendChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        FieldSetLegendChild::Time(builder.build())
    }
}
impl From<Underline> for FieldSetLegendChild {
    fn from(child: Underline) -> Self {
        FieldSetLegendChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for FieldSetLegendChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        FieldSetLegendChild::Underline(builder.build())
    }
}
impl From<Variable> for FieldSetLegendChild {
    fn from(child: Variable) -> Self {
        FieldSetLegendChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for FieldSetLegendChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        FieldSetLegendChild::Variable(builder.build())
    }
}
impl From<Video> for FieldSetLegendChild {
    fn from(child: Video) -> Self {
        FieldSetLegendChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for FieldSetLegendChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        FieldSetLegendChild::Video(builder.build())
    }
}
impl From<WordBreak> for FieldSetLegendChild {
    fn from(child: WordBreak) -> Self {
        FieldSetLegendChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for FieldSetLegendChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        FieldSetLegendChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for FieldSetLegendChild {
    fn from(s: &'static str) -> Self {
        FieldSetLegendChild::Text(s.into())
    }
}
impl From<String> for FieldSetLegendChild {
    fn from(s: String) -> Self {
        FieldSetLegendChild::Text(s.into())
    }
}
impl From<CowStr> for FieldSetLegendChild {
    fn from(s: CowStr) -> Self {
        FieldSetLegendChild::Text(s)
    }
}
impl std::fmt::Debug for FieldSetLegendChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldSetLegendChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            FieldSetLegendChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            FieldSetLegendChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Button(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Code(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Data(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Image(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Input(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Label(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Link(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Map(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Object(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Output(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Script(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Select(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Small(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Span(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Template(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Time(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Video(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            FieldSetLegendChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for FieldSetLegendChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldSetLegendChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Audio(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            FieldSetLegendChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            FieldSetLegendChild::Bold(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Button(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Cite(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Code(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Custom(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Data(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::DataList(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Definition(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Embed(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Image(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Input(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Italic(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Label(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Link(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Map(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Mark(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Meter(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Object(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Output(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Picture(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Progress(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Quote(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Sample(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Script(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Select(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Slot(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Small(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Span(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Strong(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Template(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Time(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Underline(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Variable(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Video(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            FieldSetLegendChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
