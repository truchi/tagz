// 🤖 This file is generated!

use crate::*;
/// The `<fieldset>` element's children.
#[derive(Clone)]
pub enum FieldSetChild {
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
    FieldSetLegend(FieldSetLegend),
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
impl From<Abbreviation> for FieldSetChild {
    fn from(child: Abbreviation) -> Self {
        FieldSetChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for FieldSetChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        FieldSetChild::Abbreviation(builder.build())
    }
}
impl From<Address> for FieldSetChild {
    fn from(child: Address) -> Self {
        FieldSetChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for FieldSetChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        FieldSetChild::Address(builder.build())
    }
}
impl From<Anchor> for FieldSetChild {
    fn from(child: Anchor) -> Self {
        FieldSetChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for FieldSetChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        FieldSetChild::Anchor(builder.build())
    }
}
impl From<Article> for FieldSetChild {
    fn from(child: Article) -> Self {
        FieldSetChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for FieldSetChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        FieldSetChild::Article(builder.build())
    }
}
impl From<Aside> for FieldSetChild {
    fn from(child: Aside) -> Self {
        FieldSetChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for FieldSetChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        FieldSetChild::Aside(builder.build())
    }
}
impl From<Audio> for FieldSetChild {
    fn from(child: Audio) -> Self {
        FieldSetChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for FieldSetChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        FieldSetChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for FieldSetChild {
    fn from(child: BidirectionalIsolate) -> Self {
        FieldSetChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for FieldSetChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        FieldSetChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for FieldSetChild {
    fn from(child: BidirectionalOverride) -> Self {
        FieldSetChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for FieldSetChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        FieldSetChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for FieldSetChild {
    fn from(child: BlockQuote) -> Self {
        FieldSetChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for FieldSetChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        FieldSetChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for FieldSetChild {
    fn from(child: Bold) -> Self {
        FieldSetChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for FieldSetChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        FieldSetChild::Bold(builder.build())
    }
}
impl From<Button> for FieldSetChild {
    fn from(child: Button) -> Self {
        FieldSetChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for FieldSetChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        FieldSetChild::Button(builder.build())
    }
}
impl From<Canvas> for FieldSetChild {
    fn from(child: Canvas) -> Self {
        FieldSetChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for FieldSetChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        FieldSetChild::Canvas(builder.build())
    }
}
impl From<Cite> for FieldSetChild {
    fn from(child: Cite) -> Self {
        FieldSetChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for FieldSetChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        FieldSetChild::Cite(builder.build())
    }
}
impl From<Code> for FieldSetChild {
    fn from(child: Code) -> Self {
        FieldSetChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for FieldSetChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        FieldSetChild::Code(builder.build())
    }
}
impl From<Custom> for FieldSetChild {
    fn from(child: Custom) -> Self {
        FieldSetChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for FieldSetChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        FieldSetChild::Custom(builder.build())
    }
}
impl From<Data> for FieldSetChild {
    fn from(child: Data) -> Self {
        FieldSetChild::Data(child)
    }
}
impl From<builders::DataBuilder> for FieldSetChild {
    fn from(builder: builders::DataBuilder) -> Self {
        FieldSetChild::Data(builder.build())
    }
}
impl From<DataList> for FieldSetChild {
    fn from(child: DataList) -> Self {
        FieldSetChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for FieldSetChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        FieldSetChild::DataList(builder.build())
    }
}
impl From<Definition> for FieldSetChild {
    fn from(child: Definition) -> Self {
        FieldSetChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for FieldSetChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        FieldSetChild::Definition(builder.build())
    }
}
impl From<Deleted> for FieldSetChild {
    fn from(child: Deleted) -> Self {
        FieldSetChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for FieldSetChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        FieldSetChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for FieldSetChild {
    fn from(child: DescriptionList) -> Self {
        FieldSetChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for FieldSetChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        FieldSetChild::DescriptionList(builder.build())
    }
}
impl From<Details> for FieldSetChild {
    fn from(child: Details) -> Self {
        FieldSetChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for FieldSetChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        FieldSetChild::Details(builder.build())
    }
}
impl From<Dialog> for FieldSetChild {
    fn from(child: Dialog) -> Self {
        FieldSetChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for FieldSetChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        FieldSetChild::Dialog(builder.build())
    }
}
impl From<Division> for FieldSetChild {
    fn from(child: Division) -> Self {
        FieldSetChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for FieldSetChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        FieldSetChild::Division(builder.build())
    }
}
impl From<Embed> for FieldSetChild {
    fn from(child: Embed) -> Self {
        FieldSetChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for FieldSetChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        FieldSetChild::Embed(builder.build())
    }
}
impl From<Emphasis> for FieldSetChild {
    fn from(child: Emphasis) -> Self {
        FieldSetChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for FieldSetChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        FieldSetChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for FieldSetChild {
    fn from(child: FieldSet) -> Self {
        FieldSetChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for FieldSetChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        FieldSetChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for FieldSetChild {
    fn from(child: FieldSetLegend) -> Self {
        FieldSetChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for FieldSetChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        FieldSetChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for FieldSetChild {
    fn from(child: Figure) -> Self {
        FieldSetChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for FieldSetChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        FieldSetChild::Figure(builder.build())
    }
}
impl From<Footer> for FieldSetChild {
    fn from(child: Footer) -> Self {
        FieldSetChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for FieldSetChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        FieldSetChild::Footer(builder.build())
    }
}
impl From<Form> for FieldSetChild {
    fn from(child: Form) -> Self {
        FieldSetChild::Form(child)
    }
}
impl From<builders::FormBuilder> for FieldSetChild {
    fn from(builder: builders::FormBuilder) -> Self {
        FieldSetChild::Form(builder.build())
    }
}
impl From<Header> for FieldSetChild {
    fn from(child: Header) -> Self {
        FieldSetChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for FieldSetChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        FieldSetChild::Header(builder.build())
    }
}
impl From<Heading1> for FieldSetChild {
    fn from(child: Heading1) -> Self {
        FieldSetChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for FieldSetChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        FieldSetChild::Heading1(builder.build())
    }
}
impl From<Heading2> for FieldSetChild {
    fn from(child: Heading2) -> Self {
        FieldSetChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for FieldSetChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        FieldSetChild::Heading2(builder.build())
    }
}
impl From<Heading3> for FieldSetChild {
    fn from(child: Heading3) -> Self {
        FieldSetChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for FieldSetChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        FieldSetChild::Heading3(builder.build())
    }
}
impl From<Heading4> for FieldSetChild {
    fn from(child: Heading4) -> Self {
        FieldSetChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for FieldSetChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        FieldSetChild::Heading4(builder.build())
    }
}
impl From<Heading5> for FieldSetChild {
    fn from(child: Heading5) -> Self {
        FieldSetChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for FieldSetChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        FieldSetChild::Heading5(builder.build())
    }
}
impl From<Heading6> for FieldSetChild {
    fn from(child: Heading6) -> Self {
        FieldSetChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for FieldSetChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        FieldSetChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for FieldSetChild {
    fn from(child: HeadingGroup) -> Self {
        FieldSetChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for FieldSetChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        FieldSetChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for FieldSetChild {
    fn from(child: HorizontalRule) -> Self {
        FieldSetChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for FieldSetChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        FieldSetChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for FieldSetChild {
    fn from(child: Image) -> Self {
        FieldSetChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for FieldSetChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        FieldSetChild::Image(builder.build())
    }
}
impl From<InlineFrame> for FieldSetChild {
    fn from(child: InlineFrame) -> Self {
        FieldSetChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for FieldSetChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        FieldSetChild::InlineFrame(builder.build())
    }
}
impl From<Input> for FieldSetChild {
    fn from(child: Input) -> Self {
        FieldSetChild::Input(child)
    }
}
impl From<builders::InputBuilder> for FieldSetChild {
    fn from(builder: builders::InputBuilder) -> Self {
        FieldSetChild::Input(builder.build())
    }
}
impl From<Inserted> for FieldSetChild {
    fn from(child: Inserted) -> Self {
        FieldSetChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for FieldSetChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        FieldSetChild::Inserted(builder.build())
    }
}
impl From<Italic> for FieldSetChild {
    fn from(child: Italic) -> Self {
        FieldSetChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for FieldSetChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        FieldSetChild::Italic(builder.build())
    }
}
impl From<Keyboard> for FieldSetChild {
    fn from(child: Keyboard) -> Self {
        FieldSetChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for FieldSetChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        FieldSetChild::Keyboard(builder.build())
    }
}
impl From<Label> for FieldSetChild {
    fn from(child: Label) -> Self {
        FieldSetChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for FieldSetChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        FieldSetChild::Label(builder.build())
    }
}
impl From<LineBreak> for FieldSetChild {
    fn from(child: LineBreak) -> Self {
        FieldSetChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for FieldSetChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        FieldSetChild::LineBreak(builder.build())
    }
}
impl From<Link> for FieldSetChild {
    fn from(child: Link) -> Self {
        FieldSetChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for FieldSetChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        FieldSetChild::Link(builder.build())
    }
}
impl From<Main> for FieldSetChild {
    fn from(child: Main) -> Self {
        FieldSetChild::Main(child)
    }
}
impl From<builders::MainBuilder> for FieldSetChild {
    fn from(builder: builders::MainBuilder) -> Self {
        FieldSetChild::Main(builder.build())
    }
}
impl From<Map> for FieldSetChild {
    fn from(child: Map) -> Self {
        FieldSetChild::Map(child)
    }
}
impl From<builders::MapBuilder> for FieldSetChild {
    fn from(builder: builders::MapBuilder) -> Self {
        FieldSetChild::Map(builder.build())
    }
}
impl From<MapArea> for FieldSetChild {
    fn from(child: MapArea) -> Self {
        FieldSetChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for FieldSetChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        FieldSetChild::MapArea(builder.build())
    }
}
impl From<Mark> for FieldSetChild {
    fn from(child: Mark) -> Self {
        FieldSetChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for FieldSetChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        FieldSetChild::Mark(builder.build())
    }
}
impl From<Menu> for FieldSetChild {
    fn from(child: Menu) -> Self {
        FieldSetChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for FieldSetChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        FieldSetChild::Menu(builder.build())
    }
}
impl From<Metadata> for FieldSetChild {
    fn from(child: Metadata) -> Self {
        FieldSetChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for FieldSetChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        FieldSetChild::Metadata(builder.build())
    }
}
impl From<Meter> for FieldSetChild {
    fn from(child: Meter) -> Self {
        FieldSetChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for FieldSetChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        FieldSetChild::Meter(builder.build())
    }
}
impl From<Navigation> for FieldSetChild {
    fn from(child: Navigation) -> Self {
        FieldSetChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for FieldSetChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        FieldSetChild::Navigation(builder.build())
    }
}
impl From<NoScript> for FieldSetChild {
    fn from(child: NoScript) -> Self {
        FieldSetChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for FieldSetChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        FieldSetChild::NoScript(builder.build())
    }
}
impl From<Object> for FieldSetChild {
    fn from(child: Object) -> Self {
        FieldSetChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for FieldSetChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        FieldSetChild::Object(builder.build())
    }
}
impl From<OrderedList> for FieldSetChild {
    fn from(child: OrderedList) -> Self {
        FieldSetChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for FieldSetChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        FieldSetChild::OrderedList(builder.build())
    }
}
impl From<Output> for FieldSetChild {
    fn from(child: Output) -> Self {
        FieldSetChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for FieldSetChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        FieldSetChild::Output(builder.build())
    }
}
impl From<Paragraph> for FieldSetChild {
    fn from(child: Paragraph) -> Self {
        FieldSetChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for FieldSetChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        FieldSetChild::Paragraph(builder.build())
    }
}
impl From<Picture> for FieldSetChild {
    fn from(child: Picture) -> Self {
        FieldSetChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for FieldSetChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        FieldSetChild::Picture(builder.build())
    }
}
impl From<Preformatted> for FieldSetChild {
    fn from(child: Preformatted) -> Self {
        FieldSetChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for FieldSetChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        FieldSetChild::Preformatted(builder.build())
    }
}
impl From<Progress> for FieldSetChild {
    fn from(child: Progress) -> Self {
        FieldSetChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for FieldSetChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        FieldSetChild::Progress(builder.build())
    }
}
impl From<Quote> for FieldSetChild {
    fn from(child: Quote) -> Self {
        FieldSetChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for FieldSetChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        FieldSetChild::Quote(builder.build())
    }
}
impl From<Ruby> for FieldSetChild {
    fn from(child: Ruby) -> Self {
        FieldSetChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for FieldSetChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        FieldSetChild::Ruby(builder.build())
    }
}
impl From<Sample> for FieldSetChild {
    fn from(child: Sample) -> Self {
        FieldSetChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for FieldSetChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        FieldSetChild::Sample(builder.build())
    }
}
impl From<Script> for FieldSetChild {
    fn from(child: Script) -> Self {
        FieldSetChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for FieldSetChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        FieldSetChild::Script(builder.build())
    }
}
impl From<Search> for FieldSetChild {
    fn from(child: Search) -> Self {
        FieldSetChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for FieldSetChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        FieldSetChild::Search(builder.build())
    }
}
impl From<Section> for FieldSetChild {
    fn from(child: Section) -> Self {
        FieldSetChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for FieldSetChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        FieldSetChild::Section(builder.build())
    }
}
impl From<Select> for FieldSetChild {
    fn from(child: Select) -> Self {
        FieldSetChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for FieldSetChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        FieldSetChild::Select(builder.build())
    }
}
impl From<Slot> for FieldSetChild {
    fn from(child: Slot) -> Self {
        FieldSetChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for FieldSetChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        FieldSetChild::Slot(builder.build())
    }
}
impl From<Small> for FieldSetChild {
    fn from(child: Small) -> Self {
        FieldSetChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for FieldSetChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        FieldSetChild::Small(builder.build())
    }
}
impl From<Span> for FieldSetChild {
    fn from(child: Span) -> Self {
        FieldSetChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for FieldSetChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        FieldSetChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for FieldSetChild {
    fn from(child: StrikeThrough) -> Self {
        FieldSetChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for FieldSetChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        FieldSetChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for FieldSetChild {
    fn from(child: Strong) -> Self {
        FieldSetChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for FieldSetChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        FieldSetChild::Strong(builder.build())
    }
}
impl From<SubScript> for FieldSetChild {
    fn from(child: SubScript) -> Self {
        FieldSetChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for FieldSetChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        FieldSetChild::SubScript(builder.build())
    }
}
impl From<SupScript> for FieldSetChild {
    fn from(child: SupScript) -> Self {
        FieldSetChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for FieldSetChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        FieldSetChild::SupScript(builder.build())
    }
}
impl From<Table> for FieldSetChild {
    fn from(child: Table) -> Self {
        FieldSetChild::Table(child)
    }
}
impl From<builders::TableBuilder> for FieldSetChild {
    fn from(builder: builders::TableBuilder) -> Self {
        FieldSetChild::Table(builder.build())
    }
}
impl From<Template> for FieldSetChild {
    fn from(child: Template) -> Self {
        FieldSetChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for FieldSetChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        FieldSetChild::Template(builder.build())
    }
}
impl From<TextArea> for FieldSetChild {
    fn from(child: TextArea) -> Self {
        FieldSetChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for FieldSetChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        FieldSetChild::TextArea(builder.build())
    }
}
impl From<Time> for FieldSetChild {
    fn from(child: Time) -> Self {
        FieldSetChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for FieldSetChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        FieldSetChild::Time(builder.build())
    }
}
impl From<Underline> for FieldSetChild {
    fn from(child: Underline) -> Self {
        FieldSetChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for FieldSetChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        FieldSetChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for FieldSetChild {
    fn from(child: UnorderedList) -> Self {
        FieldSetChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for FieldSetChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        FieldSetChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for FieldSetChild {
    fn from(child: Variable) -> Self {
        FieldSetChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for FieldSetChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        FieldSetChild::Variable(builder.build())
    }
}
impl From<Video> for FieldSetChild {
    fn from(child: Video) -> Self {
        FieldSetChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for FieldSetChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        FieldSetChild::Video(builder.build())
    }
}
impl From<WordBreak> for FieldSetChild {
    fn from(child: WordBreak) -> Self {
        FieldSetChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for FieldSetChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        FieldSetChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for FieldSetChild {
    fn from(s: &'static str) -> Self {
        FieldSetChild::Text(s.into())
    }
}
impl From<String> for FieldSetChild {
    fn from(s: String) -> Self {
        FieldSetChild::Text(s.into())
    }
}
impl From<CowStr> for FieldSetChild {
    fn from(s: CowStr) -> Self {
        FieldSetChild::Text(s)
    }
}
impl std::fmt::Debug for FieldSetChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldSetChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Address(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Article(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Button(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Code(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Data(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Details(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Division(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Form(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Header(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Image(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Input(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Label(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Link(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Main(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Map(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Object(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Output(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Script(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Search(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Section(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Select(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Small(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Span(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Table(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Template(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Time(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Video(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            FieldSetChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for FieldSetChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldSetChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Address(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Article(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Aside(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Audio(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            FieldSetChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            FieldSetChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Bold(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Button(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Cite(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Code(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Custom(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Data(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::DataList(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Definition(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Details(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Division(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Embed(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Figure(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Footer(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Form(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Header(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Image(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Input(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Italic(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Label(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Link(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Main(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Map(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Mark(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Menu(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Meter(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Object(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Output(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Picture(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Progress(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Quote(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Sample(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Script(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Search(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Section(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Select(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Slot(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Small(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Span(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Strong(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Table(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Template(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Time(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Underline(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Variable(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Video(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            FieldSetChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
