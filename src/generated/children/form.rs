// 🤖 This file is generated!

use crate::*;
/// The `<form>` element's children.
#[derive(Clone)]
pub enum FormChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
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
    DescriptionList(DescriptionList),
    Details(Details),
    Dialog(Dialog),
    Division(Division),
    Embed(Embed),
    Emphasis(Emphasis),
    FieldSet(FieldSet),
    Figure(Figure),
    Footer(Footer),
    Form(Form),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    Main(Main),
    Map(Map),
    MapArea(MapArea),
    Mark(Mark),
    Menu(Menu),
    Metadata(Metadata),
    Meter(Meter),
    Navigation(Navigation),
    NoScript(NoScript),
    Object(Object),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Table(Table),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for FormChild {
    fn from(child: Abbreviation) -> Self {
        FormChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for FormChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        FormChild::Abbreviation(builder.build())
    }
}
impl From<Address> for FormChild {
    fn from(child: Address) -> Self {
        FormChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for FormChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        FormChild::Address(builder.build())
    }
}
impl From<Anchor> for FormChild {
    fn from(child: Anchor) -> Self {
        FormChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for FormChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        FormChild::Anchor(builder.build())
    }
}
impl From<Article> for FormChild {
    fn from(child: Article) -> Self {
        FormChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for FormChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        FormChild::Article(builder.build())
    }
}
impl From<Aside> for FormChild {
    fn from(child: Aside) -> Self {
        FormChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for FormChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        FormChild::Aside(builder.build())
    }
}
impl From<Audio> for FormChild {
    fn from(child: Audio) -> Self {
        FormChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for FormChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        FormChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for FormChild {
    fn from(child: BidirectionalIsolate) -> Self {
        FormChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for FormChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        FormChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for FormChild {
    fn from(child: BidirectionalOverride) -> Self {
        FormChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for FormChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        FormChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for FormChild {
    fn from(child: BlockQuote) -> Self {
        FormChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for FormChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        FormChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for FormChild {
    fn from(child: Bold) -> Self {
        FormChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for FormChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        FormChild::Bold(builder.build())
    }
}
impl From<Button> for FormChild {
    fn from(child: Button) -> Self {
        FormChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for FormChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        FormChild::Button(builder.build())
    }
}
impl From<Canvas> for FormChild {
    fn from(child: Canvas) -> Self {
        FormChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for FormChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        FormChild::Canvas(builder.build())
    }
}
impl From<Cite> for FormChild {
    fn from(child: Cite) -> Self {
        FormChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for FormChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        FormChild::Cite(builder.build())
    }
}
impl From<Code> for FormChild {
    fn from(child: Code) -> Self {
        FormChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for FormChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        FormChild::Code(builder.build())
    }
}
impl From<Custom> for FormChild {
    fn from(child: Custom) -> Self {
        FormChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for FormChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        FormChild::Custom(builder.build())
    }
}
impl From<Data> for FormChild {
    fn from(child: Data) -> Self {
        FormChild::Data(child)
    }
}
impl From<builders::DataBuilder> for FormChild {
    fn from(builder: builders::DataBuilder) -> Self {
        FormChild::Data(builder.build())
    }
}
impl From<DataList> for FormChild {
    fn from(child: DataList) -> Self {
        FormChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for FormChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        FormChild::DataList(builder.build())
    }
}
impl From<Definition> for FormChild {
    fn from(child: Definition) -> Self {
        FormChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for FormChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        FormChild::Definition(builder.build())
    }
}
impl From<Deleted> for FormChild {
    fn from(child: Deleted) -> Self {
        FormChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for FormChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        FormChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for FormChild {
    fn from(child: DescriptionList) -> Self {
        FormChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for FormChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        FormChild::DescriptionList(builder.build())
    }
}
impl From<Details> for FormChild {
    fn from(child: Details) -> Self {
        FormChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for FormChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        FormChild::Details(builder.build())
    }
}
impl From<Dialog> for FormChild {
    fn from(child: Dialog) -> Self {
        FormChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for FormChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        FormChild::Dialog(builder.build())
    }
}
impl From<Division> for FormChild {
    fn from(child: Division) -> Self {
        FormChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for FormChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        FormChild::Division(builder.build())
    }
}
impl From<Embed> for FormChild {
    fn from(child: Embed) -> Self {
        FormChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for FormChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        FormChild::Embed(builder.build())
    }
}
impl From<Emphasis> for FormChild {
    fn from(child: Emphasis) -> Self {
        FormChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for FormChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        FormChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for FormChild {
    fn from(child: FieldSet) -> Self {
        FormChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for FormChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        FormChild::FieldSet(builder.build())
    }
}
impl From<Figure> for FormChild {
    fn from(child: Figure) -> Self {
        FormChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for FormChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        FormChild::Figure(builder.build())
    }
}
impl From<Footer> for FormChild {
    fn from(child: Footer) -> Self {
        FormChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for FormChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        FormChild::Footer(builder.build())
    }
}
impl From<Form> for FormChild {
    fn from(child: Form) -> Self {
        FormChild::Form(child)
    }
}
impl From<builders::FormBuilder> for FormChild {
    fn from(builder: builders::FormBuilder) -> Self {
        FormChild::Form(builder.build())
    }
}
impl From<Header> for FormChild {
    fn from(child: Header) -> Self {
        FormChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for FormChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        FormChild::Header(builder.build())
    }
}
impl From<Heading1> for FormChild {
    fn from(child: Heading1) -> Self {
        FormChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for FormChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        FormChild::Heading1(builder.build())
    }
}
impl From<Heading2> for FormChild {
    fn from(child: Heading2) -> Self {
        FormChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for FormChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        FormChild::Heading2(builder.build())
    }
}
impl From<Heading3> for FormChild {
    fn from(child: Heading3) -> Self {
        FormChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for FormChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        FormChild::Heading3(builder.build())
    }
}
impl From<Heading4> for FormChild {
    fn from(child: Heading4) -> Self {
        FormChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for FormChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        FormChild::Heading4(builder.build())
    }
}
impl From<Heading5> for FormChild {
    fn from(child: Heading5) -> Self {
        FormChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for FormChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        FormChild::Heading5(builder.build())
    }
}
impl From<Heading6> for FormChild {
    fn from(child: Heading6) -> Self {
        FormChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for FormChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        FormChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for FormChild {
    fn from(child: HeadingGroup) -> Self {
        FormChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for FormChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        FormChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for FormChild {
    fn from(child: HorizontalRule) -> Self {
        FormChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for FormChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        FormChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for FormChild {
    fn from(child: Image) -> Self {
        FormChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for FormChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        FormChild::Image(builder.build())
    }
}
impl From<InlineFrame> for FormChild {
    fn from(child: InlineFrame) -> Self {
        FormChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for FormChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        FormChild::InlineFrame(builder.build())
    }
}
impl From<Input> for FormChild {
    fn from(child: Input) -> Self {
        FormChild::Input(child)
    }
}
impl From<builders::InputBuilder> for FormChild {
    fn from(builder: builders::InputBuilder) -> Self {
        FormChild::Input(builder.build())
    }
}
impl From<Inserted> for FormChild {
    fn from(child: Inserted) -> Self {
        FormChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for FormChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        FormChild::Inserted(builder.build())
    }
}
impl From<Italic> for FormChild {
    fn from(child: Italic) -> Self {
        FormChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for FormChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        FormChild::Italic(builder.build())
    }
}
impl From<Keyboard> for FormChild {
    fn from(child: Keyboard) -> Self {
        FormChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for FormChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        FormChild::Keyboard(builder.build())
    }
}
impl From<Label> for FormChild {
    fn from(child: Label) -> Self {
        FormChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for FormChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        FormChild::Label(builder.build())
    }
}
impl From<LineBreak> for FormChild {
    fn from(child: LineBreak) -> Self {
        FormChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for FormChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        FormChild::LineBreak(builder.build())
    }
}
impl From<Link> for FormChild {
    fn from(child: Link) -> Self {
        FormChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for FormChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        FormChild::Link(builder.build())
    }
}
impl From<Main> for FormChild {
    fn from(child: Main) -> Self {
        FormChild::Main(child)
    }
}
impl From<builders::MainBuilder> for FormChild {
    fn from(builder: builders::MainBuilder) -> Self {
        FormChild::Main(builder.build())
    }
}
impl From<Map> for FormChild {
    fn from(child: Map) -> Self {
        FormChild::Map(child)
    }
}
impl From<builders::MapBuilder> for FormChild {
    fn from(builder: builders::MapBuilder) -> Self {
        FormChild::Map(builder.build())
    }
}
impl From<MapArea> for FormChild {
    fn from(child: MapArea) -> Self {
        FormChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for FormChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        FormChild::MapArea(builder.build())
    }
}
impl From<Mark> for FormChild {
    fn from(child: Mark) -> Self {
        FormChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for FormChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        FormChild::Mark(builder.build())
    }
}
impl From<Menu> for FormChild {
    fn from(child: Menu) -> Self {
        FormChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for FormChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        FormChild::Menu(builder.build())
    }
}
impl From<Metadata> for FormChild {
    fn from(child: Metadata) -> Self {
        FormChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for FormChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        FormChild::Metadata(builder.build())
    }
}
impl From<Meter> for FormChild {
    fn from(child: Meter) -> Self {
        FormChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for FormChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        FormChild::Meter(builder.build())
    }
}
impl From<Navigation> for FormChild {
    fn from(child: Navigation) -> Self {
        FormChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for FormChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        FormChild::Navigation(builder.build())
    }
}
impl From<NoScript> for FormChild {
    fn from(child: NoScript) -> Self {
        FormChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for FormChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        FormChild::NoScript(builder.build())
    }
}
impl From<Object> for FormChild {
    fn from(child: Object) -> Self {
        FormChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for FormChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        FormChild::Object(builder.build())
    }
}
impl From<OrderedList> for FormChild {
    fn from(child: OrderedList) -> Self {
        FormChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for FormChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        FormChild::OrderedList(builder.build())
    }
}
impl From<Output> for FormChild {
    fn from(child: Output) -> Self {
        FormChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for FormChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        FormChild::Output(builder.build())
    }
}
impl From<Paragraph> for FormChild {
    fn from(child: Paragraph) -> Self {
        FormChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for FormChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        FormChild::Paragraph(builder.build())
    }
}
impl From<Picture> for FormChild {
    fn from(child: Picture) -> Self {
        FormChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for FormChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        FormChild::Picture(builder.build())
    }
}
impl From<Preformatted> for FormChild {
    fn from(child: Preformatted) -> Self {
        FormChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for FormChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        FormChild::Preformatted(builder.build())
    }
}
impl From<Progress> for FormChild {
    fn from(child: Progress) -> Self {
        FormChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for FormChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        FormChild::Progress(builder.build())
    }
}
impl From<Quote> for FormChild {
    fn from(child: Quote) -> Self {
        FormChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for FormChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        FormChild::Quote(builder.build())
    }
}
impl From<Ruby> for FormChild {
    fn from(child: Ruby) -> Self {
        FormChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for FormChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        FormChild::Ruby(builder.build())
    }
}
impl From<Sample> for FormChild {
    fn from(child: Sample) -> Self {
        FormChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for FormChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        FormChild::Sample(builder.build())
    }
}
impl From<Script> for FormChild {
    fn from(child: Script) -> Self {
        FormChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for FormChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        FormChild::Script(builder.build())
    }
}
impl From<Search> for FormChild {
    fn from(child: Search) -> Self {
        FormChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for FormChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        FormChild::Search(builder.build())
    }
}
impl From<Section> for FormChild {
    fn from(child: Section) -> Self {
        FormChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for FormChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        FormChild::Section(builder.build())
    }
}
impl From<Select> for FormChild {
    fn from(child: Select) -> Self {
        FormChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for FormChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        FormChild::Select(builder.build())
    }
}
impl From<Slot> for FormChild {
    fn from(child: Slot) -> Self {
        FormChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for FormChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        FormChild::Slot(builder.build())
    }
}
impl From<Small> for FormChild {
    fn from(child: Small) -> Self {
        FormChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for FormChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        FormChild::Small(builder.build())
    }
}
impl From<Span> for FormChild {
    fn from(child: Span) -> Self {
        FormChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for FormChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        FormChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for FormChild {
    fn from(child: StrikeThrough) -> Self {
        FormChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for FormChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        FormChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for FormChild {
    fn from(child: Strong) -> Self {
        FormChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for FormChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        FormChild::Strong(builder.build())
    }
}
impl From<SubScript> for FormChild {
    fn from(child: SubScript) -> Self {
        FormChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for FormChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        FormChild::SubScript(builder.build())
    }
}
impl From<SupScript> for FormChild {
    fn from(child: SupScript) -> Self {
        FormChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for FormChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        FormChild::SupScript(builder.build())
    }
}
impl From<Table> for FormChild {
    fn from(child: Table) -> Self {
        FormChild::Table(child)
    }
}
impl From<builders::TableBuilder> for FormChild {
    fn from(builder: builders::TableBuilder) -> Self {
        FormChild::Table(builder.build())
    }
}
impl From<Template> for FormChild {
    fn from(child: Template) -> Self {
        FormChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for FormChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        FormChild::Template(builder.build())
    }
}
impl From<TextArea> for FormChild {
    fn from(child: TextArea) -> Self {
        FormChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for FormChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        FormChild::TextArea(builder.build())
    }
}
impl From<Time> for FormChild {
    fn from(child: Time) -> Self {
        FormChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for FormChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        FormChild::Time(builder.build())
    }
}
impl From<Underline> for FormChild {
    fn from(child: Underline) -> Self {
        FormChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for FormChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        FormChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for FormChild {
    fn from(child: UnorderedList) -> Self {
        FormChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for FormChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        FormChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for FormChild {
    fn from(child: Variable) -> Self {
        FormChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for FormChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        FormChild::Variable(builder.build())
    }
}
impl From<Video> for FormChild {
    fn from(child: Video) -> Self {
        FormChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for FormChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        FormChild::Video(builder.build())
    }
}
impl From<WordBreak> for FormChild {
    fn from(child: WordBreak) -> Self {
        FormChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for FormChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        FormChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for FormChild {
    fn from(s: &'static str) -> Self {
        FormChild::Text(s.into())
    }
}
impl From<String> for FormChild {
    fn from(s: String) -> Self {
        FormChild::Text(s.into())
    }
}
impl From<CowStr> for FormChild {
    fn from(s: CowStr) -> Self {
        FormChild::Text(s)
    }
}
impl std::fmt::Debug for FormChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Address(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Article(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            FormChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            FormChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            FormChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Button(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Code(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Data(child) => std::fmt::Debug::fmt(child, f),
            FormChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            FormChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Details(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Division(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            FormChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Form(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Header(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            FormChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            FormChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Image(child) => std::fmt::Debug::fmt(child, f),
            FormChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Input(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Label(child) => std::fmt::Debug::fmt(child, f),
            FormChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Link(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Main(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Map(child) => std::fmt::Debug::fmt(child, f),
            FormChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            FormChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Object(child) => std::fmt::Debug::fmt(child, f),
            FormChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Output(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Script(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Search(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Section(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Select(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Small(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Span(child) => std::fmt::Debug::fmt(child, f),
            FormChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            FormChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            FormChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Table(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Template(child) => std::fmt::Debug::fmt(child, f),
            FormChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Time(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            FormChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Video(child) => std::fmt::Debug::fmt(child, f),
            FormChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            FormChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for FormChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            FormChild::Address(child) => std::fmt::Display::fmt(child, f),
            FormChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            FormChild::Article(child) => std::fmt::Display::fmt(child, f),
            FormChild::Aside(child) => std::fmt::Display::fmt(child, f),
            FormChild::Audio(child) => std::fmt::Display::fmt(child, f),
            FormChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            FormChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            FormChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            FormChild::Bold(child) => std::fmt::Display::fmt(child, f),
            FormChild::Button(child) => std::fmt::Display::fmt(child, f),
            FormChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            FormChild::Cite(child) => std::fmt::Display::fmt(child, f),
            FormChild::Code(child) => std::fmt::Display::fmt(child, f),
            FormChild::Custom(child) => std::fmt::Display::fmt(child, f),
            FormChild::Data(child) => std::fmt::Display::fmt(child, f),
            FormChild::DataList(child) => std::fmt::Display::fmt(child, f),
            FormChild::Definition(child) => std::fmt::Display::fmt(child, f),
            FormChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            FormChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            FormChild::Details(child) => std::fmt::Display::fmt(child, f),
            FormChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            FormChild::Division(child) => std::fmt::Display::fmt(child, f),
            FormChild::Embed(child) => std::fmt::Display::fmt(child, f),
            FormChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            FormChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            FormChild::Figure(child) => std::fmt::Display::fmt(child, f),
            FormChild::Footer(child) => std::fmt::Display::fmt(child, f),
            FormChild::Form(child) => std::fmt::Display::fmt(child, f),
            FormChild::Header(child) => std::fmt::Display::fmt(child, f),
            FormChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            FormChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            FormChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            FormChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            FormChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            FormChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            FormChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            FormChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            FormChild::Image(child) => std::fmt::Display::fmt(child, f),
            FormChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            FormChild::Input(child) => std::fmt::Display::fmt(child, f),
            FormChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            FormChild::Italic(child) => std::fmt::Display::fmt(child, f),
            FormChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            FormChild::Label(child) => std::fmt::Display::fmt(child, f),
            FormChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            FormChild::Link(child) => std::fmt::Display::fmt(child, f),
            FormChild::Main(child) => std::fmt::Display::fmt(child, f),
            FormChild::Map(child) => std::fmt::Display::fmt(child, f),
            FormChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            FormChild::Mark(child) => std::fmt::Display::fmt(child, f),
            FormChild::Menu(child) => std::fmt::Display::fmt(child, f),
            FormChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            FormChild::Meter(child) => std::fmt::Display::fmt(child, f),
            FormChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            FormChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            FormChild::Object(child) => std::fmt::Display::fmt(child, f),
            FormChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            FormChild::Output(child) => std::fmt::Display::fmt(child, f),
            FormChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            FormChild::Picture(child) => std::fmt::Display::fmt(child, f),
            FormChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            FormChild::Progress(child) => std::fmt::Display::fmt(child, f),
            FormChild::Quote(child) => std::fmt::Display::fmt(child, f),
            FormChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            FormChild::Sample(child) => std::fmt::Display::fmt(child, f),
            FormChild::Script(child) => std::fmt::Display::fmt(child, f),
            FormChild::Search(child) => std::fmt::Display::fmt(child, f),
            FormChild::Section(child) => std::fmt::Display::fmt(child, f),
            FormChild::Select(child) => std::fmt::Display::fmt(child, f),
            FormChild::Slot(child) => std::fmt::Display::fmt(child, f),
            FormChild::Small(child) => std::fmt::Display::fmt(child, f),
            FormChild::Span(child) => std::fmt::Display::fmt(child, f),
            FormChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            FormChild::Strong(child) => std::fmt::Display::fmt(child, f),
            FormChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            FormChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            FormChild::Table(child) => std::fmt::Display::fmt(child, f),
            FormChild::Template(child) => std::fmt::Display::fmt(child, f),
            FormChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            FormChild::Time(child) => std::fmt::Display::fmt(child, f),
            FormChild::Underline(child) => std::fmt::Display::fmt(child, f),
            FormChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            FormChild::Variable(child) => std::fmt::Display::fmt(child, f),
            FormChild::Video(child) => std::fmt::Display::fmt(child, f),
            FormChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            FormChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
