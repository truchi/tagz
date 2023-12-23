// 🤖 This file is generated!

use crate::*;
/// The `<dd>` element's children.
#[derive(Clone)]
pub enum DescriptionDetailsChild {
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
impl From<Abbreviation> for DescriptionDetailsChild {
    fn from(child: Abbreviation) -> Self {
        DescriptionDetailsChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        DescriptionDetailsChild::Abbreviation(builder.build())
    }
}
impl From<Address> for DescriptionDetailsChild {
    fn from(child: Address) -> Self {
        DescriptionDetailsChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        DescriptionDetailsChild::Address(builder.build())
    }
}
impl From<Anchor> for DescriptionDetailsChild {
    fn from(child: Anchor) -> Self {
        DescriptionDetailsChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        DescriptionDetailsChild::Anchor(builder.build())
    }
}
impl From<Article> for DescriptionDetailsChild {
    fn from(child: Article) -> Self {
        DescriptionDetailsChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        DescriptionDetailsChild::Article(builder.build())
    }
}
impl From<Aside> for DescriptionDetailsChild {
    fn from(child: Aside) -> Self {
        DescriptionDetailsChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        DescriptionDetailsChild::Aside(builder.build())
    }
}
impl From<Audio> for DescriptionDetailsChild {
    fn from(child: Audio) -> Self {
        DescriptionDetailsChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        DescriptionDetailsChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for DescriptionDetailsChild {
    fn from(child: BidirectionalIsolate) -> Self {
        DescriptionDetailsChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        DescriptionDetailsChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for DescriptionDetailsChild {
    fn from(child: BidirectionalOverride) -> Self {
        DescriptionDetailsChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        DescriptionDetailsChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for DescriptionDetailsChild {
    fn from(child: BlockQuote) -> Self {
        DescriptionDetailsChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        DescriptionDetailsChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for DescriptionDetailsChild {
    fn from(child: Bold) -> Self {
        DescriptionDetailsChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        DescriptionDetailsChild::Bold(builder.build())
    }
}
impl From<Button> for DescriptionDetailsChild {
    fn from(child: Button) -> Self {
        DescriptionDetailsChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        DescriptionDetailsChild::Button(builder.build())
    }
}
impl From<Canvas> for DescriptionDetailsChild {
    fn from(child: Canvas) -> Self {
        DescriptionDetailsChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        DescriptionDetailsChild::Canvas(builder.build())
    }
}
impl From<Cite> for DescriptionDetailsChild {
    fn from(child: Cite) -> Self {
        DescriptionDetailsChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        DescriptionDetailsChild::Cite(builder.build())
    }
}
impl From<Code> for DescriptionDetailsChild {
    fn from(child: Code) -> Self {
        DescriptionDetailsChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        DescriptionDetailsChild::Code(builder.build())
    }
}
impl From<Custom> for DescriptionDetailsChild {
    fn from(child: Custom) -> Self {
        DescriptionDetailsChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        DescriptionDetailsChild::Custom(builder.build())
    }
}
impl From<Data> for DescriptionDetailsChild {
    fn from(child: Data) -> Self {
        DescriptionDetailsChild::Data(child)
    }
}
impl From<builders::DataBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::DataBuilder) -> Self {
        DescriptionDetailsChild::Data(builder.build())
    }
}
impl From<DataList> for DescriptionDetailsChild {
    fn from(child: DataList) -> Self {
        DescriptionDetailsChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        DescriptionDetailsChild::DataList(builder.build())
    }
}
impl From<Definition> for DescriptionDetailsChild {
    fn from(child: Definition) -> Self {
        DescriptionDetailsChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        DescriptionDetailsChild::Definition(builder.build())
    }
}
impl From<Deleted> for DescriptionDetailsChild {
    fn from(child: Deleted) -> Self {
        DescriptionDetailsChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        DescriptionDetailsChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for DescriptionDetailsChild {
    fn from(child: DescriptionList) -> Self {
        DescriptionDetailsChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        DescriptionDetailsChild::DescriptionList(builder.build())
    }
}
impl From<Details> for DescriptionDetailsChild {
    fn from(child: Details) -> Self {
        DescriptionDetailsChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        DescriptionDetailsChild::Details(builder.build())
    }
}
impl From<Dialog> for DescriptionDetailsChild {
    fn from(child: Dialog) -> Self {
        DescriptionDetailsChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        DescriptionDetailsChild::Dialog(builder.build())
    }
}
impl From<Division> for DescriptionDetailsChild {
    fn from(child: Division) -> Self {
        DescriptionDetailsChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        DescriptionDetailsChild::Division(builder.build())
    }
}
impl From<Embed> for DescriptionDetailsChild {
    fn from(child: Embed) -> Self {
        DescriptionDetailsChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        DescriptionDetailsChild::Embed(builder.build())
    }
}
impl From<Emphasis> for DescriptionDetailsChild {
    fn from(child: Emphasis) -> Self {
        DescriptionDetailsChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        DescriptionDetailsChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for DescriptionDetailsChild {
    fn from(child: FieldSet) -> Self {
        DescriptionDetailsChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        DescriptionDetailsChild::FieldSet(builder.build())
    }
}
impl From<Figure> for DescriptionDetailsChild {
    fn from(child: Figure) -> Self {
        DescriptionDetailsChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        DescriptionDetailsChild::Figure(builder.build())
    }
}
impl From<Footer> for DescriptionDetailsChild {
    fn from(child: Footer) -> Self {
        DescriptionDetailsChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        DescriptionDetailsChild::Footer(builder.build())
    }
}
impl From<Form> for DescriptionDetailsChild {
    fn from(child: Form) -> Self {
        DescriptionDetailsChild::Form(child)
    }
}
impl From<builders::FormBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::FormBuilder) -> Self {
        DescriptionDetailsChild::Form(builder.build())
    }
}
impl From<Header> for DescriptionDetailsChild {
    fn from(child: Header) -> Self {
        DescriptionDetailsChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        DescriptionDetailsChild::Header(builder.build())
    }
}
impl From<Heading1> for DescriptionDetailsChild {
    fn from(child: Heading1) -> Self {
        DescriptionDetailsChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for DescriptionDetailsChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        DescriptionDetailsChild::Heading1(builder.build())
    }
}
impl From<Heading2> for DescriptionDetailsChild {
    fn from(child: Heading2) -> Self {
        DescriptionDetailsChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for DescriptionDetailsChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        DescriptionDetailsChild::Heading2(builder.build())
    }
}
impl From<Heading3> for DescriptionDetailsChild {
    fn from(child: Heading3) -> Self {
        DescriptionDetailsChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for DescriptionDetailsChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        DescriptionDetailsChild::Heading3(builder.build())
    }
}
impl From<Heading4> for DescriptionDetailsChild {
    fn from(child: Heading4) -> Self {
        DescriptionDetailsChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for DescriptionDetailsChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        DescriptionDetailsChild::Heading4(builder.build())
    }
}
impl From<Heading5> for DescriptionDetailsChild {
    fn from(child: Heading5) -> Self {
        DescriptionDetailsChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for DescriptionDetailsChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        DescriptionDetailsChild::Heading5(builder.build())
    }
}
impl From<Heading6> for DescriptionDetailsChild {
    fn from(child: Heading6) -> Self {
        DescriptionDetailsChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for DescriptionDetailsChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        DescriptionDetailsChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for DescriptionDetailsChild {
    fn from(child: HeadingGroup) -> Self {
        DescriptionDetailsChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        DescriptionDetailsChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for DescriptionDetailsChild {
    fn from(child: HorizontalRule) -> Self {
        DescriptionDetailsChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        DescriptionDetailsChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for DescriptionDetailsChild {
    fn from(child: Image) -> Self {
        DescriptionDetailsChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        DescriptionDetailsChild::Image(builder.build())
    }
}
impl From<InlineFrame> for DescriptionDetailsChild {
    fn from(child: InlineFrame) -> Self {
        DescriptionDetailsChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        DescriptionDetailsChild::InlineFrame(builder.build())
    }
}
impl From<Input> for DescriptionDetailsChild {
    fn from(child: Input) -> Self {
        DescriptionDetailsChild::Input(child)
    }
}
impl From<builders::InputBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::InputBuilder) -> Self {
        DescriptionDetailsChild::Input(builder.build())
    }
}
impl From<Inserted> for DescriptionDetailsChild {
    fn from(child: Inserted) -> Self {
        DescriptionDetailsChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        DescriptionDetailsChild::Inserted(builder.build())
    }
}
impl From<Italic> for DescriptionDetailsChild {
    fn from(child: Italic) -> Self {
        DescriptionDetailsChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        DescriptionDetailsChild::Italic(builder.build())
    }
}
impl From<Keyboard> for DescriptionDetailsChild {
    fn from(child: Keyboard) -> Self {
        DescriptionDetailsChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        DescriptionDetailsChild::Keyboard(builder.build())
    }
}
impl From<Label> for DescriptionDetailsChild {
    fn from(child: Label) -> Self {
        DescriptionDetailsChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        DescriptionDetailsChild::Label(builder.build())
    }
}
impl From<LineBreak> for DescriptionDetailsChild {
    fn from(child: LineBreak) -> Self {
        DescriptionDetailsChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        DescriptionDetailsChild::LineBreak(builder.build())
    }
}
impl From<Link> for DescriptionDetailsChild {
    fn from(child: Link) -> Self {
        DescriptionDetailsChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        DescriptionDetailsChild::Link(builder.build())
    }
}
impl From<Main> for DescriptionDetailsChild {
    fn from(child: Main) -> Self {
        DescriptionDetailsChild::Main(child)
    }
}
impl From<builders::MainBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::MainBuilder) -> Self {
        DescriptionDetailsChild::Main(builder.build())
    }
}
impl From<Map> for DescriptionDetailsChild {
    fn from(child: Map) -> Self {
        DescriptionDetailsChild::Map(child)
    }
}
impl From<builders::MapBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::MapBuilder) -> Self {
        DescriptionDetailsChild::Map(builder.build())
    }
}
impl From<MapArea> for DescriptionDetailsChild {
    fn from(child: MapArea) -> Self {
        DescriptionDetailsChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        DescriptionDetailsChild::MapArea(builder.build())
    }
}
impl From<Mark> for DescriptionDetailsChild {
    fn from(child: Mark) -> Self {
        DescriptionDetailsChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        DescriptionDetailsChild::Mark(builder.build())
    }
}
impl From<Menu> for DescriptionDetailsChild {
    fn from(child: Menu) -> Self {
        DescriptionDetailsChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        DescriptionDetailsChild::Menu(builder.build())
    }
}
impl From<Metadata> for DescriptionDetailsChild {
    fn from(child: Metadata) -> Self {
        DescriptionDetailsChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        DescriptionDetailsChild::Metadata(builder.build())
    }
}
impl From<Meter> for DescriptionDetailsChild {
    fn from(child: Meter) -> Self {
        DescriptionDetailsChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        DescriptionDetailsChild::Meter(builder.build())
    }
}
impl From<Navigation> for DescriptionDetailsChild {
    fn from(child: Navigation) -> Self {
        DescriptionDetailsChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        DescriptionDetailsChild::Navigation(builder.build())
    }
}
impl From<NoScript> for DescriptionDetailsChild {
    fn from(child: NoScript) -> Self {
        DescriptionDetailsChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        DescriptionDetailsChild::NoScript(builder.build())
    }
}
impl From<Object> for DescriptionDetailsChild {
    fn from(child: Object) -> Self {
        DescriptionDetailsChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        DescriptionDetailsChild::Object(builder.build())
    }
}
impl From<OrderedList> for DescriptionDetailsChild {
    fn from(child: OrderedList) -> Self {
        DescriptionDetailsChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        DescriptionDetailsChild::OrderedList(builder.build())
    }
}
impl From<Output> for DescriptionDetailsChild {
    fn from(child: Output) -> Self {
        DescriptionDetailsChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        DescriptionDetailsChild::Output(builder.build())
    }
}
impl From<Paragraph> for DescriptionDetailsChild {
    fn from(child: Paragraph) -> Self {
        DescriptionDetailsChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        DescriptionDetailsChild::Paragraph(builder.build())
    }
}
impl From<Picture> for DescriptionDetailsChild {
    fn from(child: Picture) -> Self {
        DescriptionDetailsChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        DescriptionDetailsChild::Picture(builder.build())
    }
}
impl From<Preformatted> for DescriptionDetailsChild {
    fn from(child: Preformatted) -> Self {
        DescriptionDetailsChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        DescriptionDetailsChild::Preformatted(builder.build())
    }
}
impl From<Progress> for DescriptionDetailsChild {
    fn from(child: Progress) -> Self {
        DescriptionDetailsChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        DescriptionDetailsChild::Progress(builder.build())
    }
}
impl From<Quote> for DescriptionDetailsChild {
    fn from(child: Quote) -> Self {
        DescriptionDetailsChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        DescriptionDetailsChild::Quote(builder.build())
    }
}
impl From<Ruby> for DescriptionDetailsChild {
    fn from(child: Ruby) -> Self {
        DescriptionDetailsChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        DescriptionDetailsChild::Ruby(builder.build())
    }
}
impl From<Sample> for DescriptionDetailsChild {
    fn from(child: Sample) -> Self {
        DescriptionDetailsChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        DescriptionDetailsChild::Sample(builder.build())
    }
}
impl From<Script> for DescriptionDetailsChild {
    fn from(child: Script) -> Self {
        DescriptionDetailsChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DescriptionDetailsChild::Script(builder.build())
    }
}
impl From<Search> for DescriptionDetailsChild {
    fn from(child: Search) -> Self {
        DescriptionDetailsChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        DescriptionDetailsChild::Search(builder.build())
    }
}
impl From<Section> for DescriptionDetailsChild {
    fn from(child: Section) -> Self {
        DescriptionDetailsChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        DescriptionDetailsChild::Section(builder.build())
    }
}
impl From<Select> for DescriptionDetailsChild {
    fn from(child: Select) -> Self {
        DescriptionDetailsChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        DescriptionDetailsChild::Select(builder.build())
    }
}
impl From<Slot> for DescriptionDetailsChild {
    fn from(child: Slot) -> Self {
        DescriptionDetailsChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        DescriptionDetailsChild::Slot(builder.build())
    }
}
impl From<Small> for DescriptionDetailsChild {
    fn from(child: Small) -> Self {
        DescriptionDetailsChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        DescriptionDetailsChild::Small(builder.build())
    }
}
impl From<Span> for DescriptionDetailsChild {
    fn from(child: Span) -> Self {
        DescriptionDetailsChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        DescriptionDetailsChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for DescriptionDetailsChild {
    fn from(child: StrikeThrough) -> Self {
        DescriptionDetailsChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        DescriptionDetailsChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for DescriptionDetailsChild {
    fn from(child: Strong) -> Self {
        DescriptionDetailsChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        DescriptionDetailsChild::Strong(builder.build())
    }
}
impl From<SubScript> for DescriptionDetailsChild {
    fn from(child: SubScript) -> Self {
        DescriptionDetailsChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        DescriptionDetailsChild::SubScript(builder.build())
    }
}
impl From<SupScript> for DescriptionDetailsChild {
    fn from(child: SupScript) -> Self {
        DescriptionDetailsChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        DescriptionDetailsChild::SupScript(builder.build())
    }
}
impl From<Table> for DescriptionDetailsChild {
    fn from(child: Table) -> Self {
        DescriptionDetailsChild::Table(child)
    }
}
impl From<builders::TableBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::TableBuilder) -> Self {
        DescriptionDetailsChild::Table(builder.build())
    }
}
impl From<Template> for DescriptionDetailsChild {
    fn from(child: Template) -> Self {
        DescriptionDetailsChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DescriptionDetailsChild::Template(builder.build())
    }
}
impl From<TextArea> for DescriptionDetailsChild {
    fn from(child: TextArea) -> Self {
        DescriptionDetailsChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        DescriptionDetailsChild::TextArea(builder.build())
    }
}
impl From<Time> for DescriptionDetailsChild {
    fn from(child: Time) -> Self {
        DescriptionDetailsChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        DescriptionDetailsChild::Time(builder.build())
    }
}
impl From<Underline> for DescriptionDetailsChild {
    fn from(child: Underline) -> Self {
        DescriptionDetailsChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        DescriptionDetailsChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for DescriptionDetailsChild {
    fn from(child: UnorderedList) -> Self {
        DescriptionDetailsChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        DescriptionDetailsChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for DescriptionDetailsChild {
    fn from(child: Variable) -> Self {
        DescriptionDetailsChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        DescriptionDetailsChild::Variable(builder.build())
    }
}
impl From<Video> for DescriptionDetailsChild {
    fn from(child: Video) -> Self {
        DescriptionDetailsChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        DescriptionDetailsChild::Video(builder.build())
    }
}
impl From<WordBreak> for DescriptionDetailsChild {
    fn from(child: WordBreak) -> Self {
        DescriptionDetailsChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for DescriptionDetailsChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        DescriptionDetailsChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for DescriptionDetailsChild {
    fn from(s: &'static str) -> Self {
        DescriptionDetailsChild::Text(s.into())
    }
}
impl From<String> for DescriptionDetailsChild {
    fn from(s: String) -> Self {
        DescriptionDetailsChild::Text(s.into())
    }
}
impl From<CowStr> for DescriptionDetailsChild {
    fn from(s: CowStr) -> Self {
        DescriptionDetailsChild::Text(s)
    }
}
impl std::fmt::Debug for DescriptionDetailsChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DescriptionDetailsChild::Abbreviation(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionDetailsChild::Address(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Article(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionDetailsChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionDetailsChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Button(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Code(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Data(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::DescriptionList(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionDetailsChild::Details(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Division(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Form(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Header(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::HeadingGroup(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionDetailsChild::HorizontalRule(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionDetailsChild::Image(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Input(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Label(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Link(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Main(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Map(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Object(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Output(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Preformatted(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionDetailsChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Search(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Section(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Select(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Small(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Span(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::StrikeThrough(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionDetailsChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Table(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Template(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Time(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::UnorderedList(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionDetailsChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Video(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            DescriptionDetailsChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for DescriptionDetailsChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DescriptionDetailsChild::Abbreviation(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Address(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Article(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Aside(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Audio(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::BlockQuote(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Bold(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Button(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Cite(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Code(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Custom(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Data(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::DataList(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Definition(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::DescriptionList(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Details(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Division(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Embed(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Figure(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Footer(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Form(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Header(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::HeadingGroup(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::HorizontalRule(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Image(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::InlineFrame(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Input(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Italic(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Label(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Link(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Main(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Map(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Mark(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Menu(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Meter(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Navigation(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Object(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::OrderedList(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Output(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Picture(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Preformatted(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Progress(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Quote(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Sample(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Script(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Search(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Section(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Select(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Slot(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Small(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Span(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::StrikeThrough(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Strong(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Table(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Template(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Time(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Underline(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::UnorderedList(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionDetailsChild::Variable(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Video(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            DescriptionDetailsChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
