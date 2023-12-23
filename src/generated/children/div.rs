// 🤖 This file is generated!

use crate::*;
/// The `<div>` element's children.
#[derive(Clone)]
pub enum DivisionChild {
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
    DescriptionDetails(DescriptionDetails),
    DescriptionList(DescriptionList),
    DescriptionTerm(DescriptionTerm),
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
impl From<Abbreviation> for DivisionChild {
    fn from(child: Abbreviation) -> Self {
        DivisionChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for DivisionChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        DivisionChild::Abbreviation(builder.build())
    }
}
impl From<Address> for DivisionChild {
    fn from(child: Address) -> Self {
        DivisionChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for DivisionChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        DivisionChild::Address(builder.build())
    }
}
impl From<Anchor> for DivisionChild {
    fn from(child: Anchor) -> Self {
        DivisionChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for DivisionChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        DivisionChild::Anchor(builder.build())
    }
}
impl From<Article> for DivisionChild {
    fn from(child: Article) -> Self {
        DivisionChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for DivisionChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        DivisionChild::Article(builder.build())
    }
}
impl From<Aside> for DivisionChild {
    fn from(child: Aside) -> Self {
        DivisionChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for DivisionChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        DivisionChild::Aside(builder.build())
    }
}
impl From<Audio> for DivisionChild {
    fn from(child: Audio) -> Self {
        DivisionChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for DivisionChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        DivisionChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for DivisionChild {
    fn from(child: BidirectionalIsolate) -> Self {
        DivisionChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for DivisionChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        DivisionChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for DivisionChild {
    fn from(child: BidirectionalOverride) -> Self {
        DivisionChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for DivisionChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        DivisionChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for DivisionChild {
    fn from(child: BlockQuote) -> Self {
        DivisionChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for DivisionChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        DivisionChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for DivisionChild {
    fn from(child: Bold) -> Self {
        DivisionChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for DivisionChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        DivisionChild::Bold(builder.build())
    }
}
impl From<Button> for DivisionChild {
    fn from(child: Button) -> Self {
        DivisionChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for DivisionChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        DivisionChild::Button(builder.build())
    }
}
impl From<Canvas> for DivisionChild {
    fn from(child: Canvas) -> Self {
        DivisionChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for DivisionChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        DivisionChild::Canvas(builder.build())
    }
}
impl From<Cite> for DivisionChild {
    fn from(child: Cite) -> Self {
        DivisionChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for DivisionChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        DivisionChild::Cite(builder.build())
    }
}
impl From<Code> for DivisionChild {
    fn from(child: Code) -> Self {
        DivisionChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for DivisionChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        DivisionChild::Code(builder.build())
    }
}
impl From<Custom> for DivisionChild {
    fn from(child: Custom) -> Self {
        DivisionChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for DivisionChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        DivisionChild::Custom(builder.build())
    }
}
impl From<Data> for DivisionChild {
    fn from(child: Data) -> Self {
        DivisionChild::Data(child)
    }
}
impl From<builders::DataBuilder> for DivisionChild {
    fn from(builder: builders::DataBuilder) -> Self {
        DivisionChild::Data(builder.build())
    }
}
impl From<DataList> for DivisionChild {
    fn from(child: DataList) -> Self {
        DivisionChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for DivisionChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        DivisionChild::DataList(builder.build())
    }
}
impl From<Definition> for DivisionChild {
    fn from(child: Definition) -> Self {
        DivisionChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for DivisionChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        DivisionChild::Definition(builder.build())
    }
}
impl From<Deleted> for DivisionChild {
    fn from(child: Deleted) -> Self {
        DivisionChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for DivisionChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        DivisionChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for DivisionChild {
    fn from(child: DescriptionDetails) -> Self {
        DivisionChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for DivisionChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        DivisionChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for DivisionChild {
    fn from(child: DescriptionList) -> Self {
        DivisionChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for DivisionChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        DivisionChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for DivisionChild {
    fn from(child: DescriptionTerm) -> Self {
        DivisionChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for DivisionChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        DivisionChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for DivisionChild {
    fn from(child: Details) -> Self {
        DivisionChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for DivisionChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        DivisionChild::Details(builder.build())
    }
}
impl From<Dialog> for DivisionChild {
    fn from(child: Dialog) -> Self {
        DivisionChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for DivisionChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        DivisionChild::Dialog(builder.build())
    }
}
impl From<Division> for DivisionChild {
    fn from(child: Division) -> Self {
        DivisionChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for DivisionChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        DivisionChild::Division(builder.build())
    }
}
impl From<Embed> for DivisionChild {
    fn from(child: Embed) -> Self {
        DivisionChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for DivisionChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        DivisionChild::Embed(builder.build())
    }
}
impl From<Emphasis> for DivisionChild {
    fn from(child: Emphasis) -> Self {
        DivisionChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for DivisionChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        DivisionChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for DivisionChild {
    fn from(child: FieldSet) -> Self {
        DivisionChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for DivisionChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        DivisionChild::FieldSet(builder.build())
    }
}
impl From<Figure> for DivisionChild {
    fn from(child: Figure) -> Self {
        DivisionChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for DivisionChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        DivisionChild::Figure(builder.build())
    }
}
impl From<Footer> for DivisionChild {
    fn from(child: Footer) -> Self {
        DivisionChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for DivisionChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        DivisionChild::Footer(builder.build())
    }
}
impl From<Form> for DivisionChild {
    fn from(child: Form) -> Self {
        DivisionChild::Form(child)
    }
}
impl From<builders::FormBuilder> for DivisionChild {
    fn from(builder: builders::FormBuilder) -> Self {
        DivisionChild::Form(builder.build())
    }
}
impl From<Header> for DivisionChild {
    fn from(child: Header) -> Self {
        DivisionChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for DivisionChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        DivisionChild::Header(builder.build())
    }
}
impl From<Heading1> for DivisionChild {
    fn from(child: Heading1) -> Self {
        DivisionChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for DivisionChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        DivisionChild::Heading1(builder.build())
    }
}
impl From<Heading2> for DivisionChild {
    fn from(child: Heading2) -> Self {
        DivisionChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for DivisionChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        DivisionChild::Heading2(builder.build())
    }
}
impl From<Heading3> for DivisionChild {
    fn from(child: Heading3) -> Self {
        DivisionChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for DivisionChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        DivisionChild::Heading3(builder.build())
    }
}
impl From<Heading4> for DivisionChild {
    fn from(child: Heading4) -> Self {
        DivisionChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for DivisionChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        DivisionChild::Heading4(builder.build())
    }
}
impl From<Heading5> for DivisionChild {
    fn from(child: Heading5) -> Self {
        DivisionChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for DivisionChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        DivisionChild::Heading5(builder.build())
    }
}
impl From<Heading6> for DivisionChild {
    fn from(child: Heading6) -> Self {
        DivisionChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for DivisionChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        DivisionChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for DivisionChild {
    fn from(child: HeadingGroup) -> Self {
        DivisionChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for DivisionChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        DivisionChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for DivisionChild {
    fn from(child: HorizontalRule) -> Self {
        DivisionChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for DivisionChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        DivisionChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for DivisionChild {
    fn from(child: Image) -> Self {
        DivisionChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for DivisionChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        DivisionChild::Image(builder.build())
    }
}
impl From<InlineFrame> for DivisionChild {
    fn from(child: InlineFrame) -> Self {
        DivisionChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for DivisionChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        DivisionChild::InlineFrame(builder.build())
    }
}
impl From<Input> for DivisionChild {
    fn from(child: Input) -> Self {
        DivisionChild::Input(child)
    }
}
impl From<builders::InputBuilder> for DivisionChild {
    fn from(builder: builders::InputBuilder) -> Self {
        DivisionChild::Input(builder.build())
    }
}
impl From<Inserted> for DivisionChild {
    fn from(child: Inserted) -> Self {
        DivisionChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for DivisionChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        DivisionChild::Inserted(builder.build())
    }
}
impl From<Italic> for DivisionChild {
    fn from(child: Italic) -> Self {
        DivisionChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for DivisionChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        DivisionChild::Italic(builder.build())
    }
}
impl From<Keyboard> for DivisionChild {
    fn from(child: Keyboard) -> Self {
        DivisionChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for DivisionChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        DivisionChild::Keyboard(builder.build())
    }
}
impl From<Label> for DivisionChild {
    fn from(child: Label) -> Self {
        DivisionChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for DivisionChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        DivisionChild::Label(builder.build())
    }
}
impl From<LineBreak> for DivisionChild {
    fn from(child: LineBreak) -> Self {
        DivisionChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for DivisionChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        DivisionChild::LineBreak(builder.build())
    }
}
impl From<Link> for DivisionChild {
    fn from(child: Link) -> Self {
        DivisionChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for DivisionChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        DivisionChild::Link(builder.build())
    }
}
impl From<Main> for DivisionChild {
    fn from(child: Main) -> Self {
        DivisionChild::Main(child)
    }
}
impl From<builders::MainBuilder> for DivisionChild {
    fn from(builder: builders::MainBuilder) -> Self {
        DivisionChild::Main(builder.build())
    }
}
impl From<Map> for DivisionChild {
    fn from(child: Map) -> Self {
        DivisionChild::Map(child)
    }
}
impl From<builders::MapBuilder> for DivisionChild {
    fn from(builder: builders::MapBuilder) -> Self {
        DivisionChild::Map(builder.build())
    }
}
impl From<MapArea> for DivisionChild {
    fn from(child: MapArea) -> Self {
        DivisionChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for DivisionChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        DivisionChild::MapArea(builder.build())
    }
}
impl From<Mark> for DivisionChild {
    fn from(child: Mark) -> Self {
        DivisionChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for DivisionChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        DivisionChild::Mark(builder.build())
    }
}
impl From<Menu> for DivisionChild {
    fn from(child: Menu) -> Self {
        DivisionChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for DivisionChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        DivisionChild::Menu(builder.build())
    }
}
impl From<Metadata> for DivisionChild {
    fn from(child: Metadata) -> Self {
        DivisionChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for DivisionChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        DivisionChild::Metadata(builder.build())
    }
}
impl From<Meter> for DivisionChild {
    fn from(child: Meter) -> Self {
        DivisionChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for DivisionChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        DivisionChild::Meter(builder.build())
    }
}
impl From<Navigation> for DivisionChild {
    fn from(child: Navigation) -> Self {
        DivisionChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for DivisionChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        DivisionChild::Navigation(builder.build())
    }
}
impl From<NoScript> for DivisionChild {
    fn from(child: NoScript) -> Self {
        DivisionChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for DivisionChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        DivisionChild::NoScript(builder.build())
    }
}
impl From<Object> for DivisionChild {
    fn from(child: Object) -> Self {
        DivisionChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for DivisionChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        DivisionChild::Object(builder.build())
    }
}
impl From<OrderedList> for DivisionChild {
    fn from(child: OrderedList) -> Self {
        DivisionChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for DivisionChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        DivisionChild::OrderedList(builder.build())
    }
}
impl From<Output> for DivisionChild {
    fn from(child: Output) -> Self {
        DivisionChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for DivisionChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        DivisionChild::Output(builder.build())
    }
}
impl From<Paragraph> for DivisionChild {
    fn from(child: Paragraph) -> Self {
        DivisionChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for DivisionChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        DivisionChild::Paragraph(builder.build())
    }
}
impl From<Picture> for DivisionChild {
    fn from(child: Picture) -> Self {
        DivisionChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for DivisionChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        DivisionChild::Picture(builder.build())
    }
}
impl From<Preformatted> for DivisionChild {
    fn from(child: Preformatted) -> Self {
        DivisionChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for DivisionChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        DivisionChild::Preformatted(builder.build())
    }
}
impl From<Progress> for DivisionChild {
    fn from(child: Progress) -> Self {
        DivisionChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for DivisionChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        DivisionChild::Progress(builder.build())
    }
}
impl From<Quote> for DivisionChild {
    fn from(child: Quote) -> Self {
        DivisionChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for DivisionChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        DivisionChild::Quote(builder.build())
    }
}
impl From<Ruby> for DivisionChild {
    fn from(child: Ruby) -> Self {
        DivisionChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for DivisionChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        DivisionChild::Ruby(builder.build())
    }
}
impl From<Sample> for DivisionChild {
    fn from(child: Sample) -> Self {
        DivisionChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for DivisionChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        DivisionChild::Sample(builder.build())
    }
}
impl From<Script> for DivisionChild {
    fn from(child: Script) -> Self {
        DivisionChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DivisionChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DivisionChild::Script(builder.build())
    }
}
impl From<Search> for DivisionChild {
    fn from(child: Search) -> Self {
        DivisionChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for DivisionChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        DivisionChild::Search(builder.build())
    }
}
impl From<Section> for DivisionChild {
    fn from(child: Section) -> Self {
        DivisionChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for DivisionChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        DivisionChild::Section(builder.build())
    }
}
impl From<Select> for DivisionChild {
    fn from(child: Select) -> Self {
        DivisionChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for DivisionChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        DivisionChild::Select(builder.build())
    }
}
impl From<Slot> for DivisionChild {
    fn from(child: Slot) -> Self {
        DivisionChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for DivisionChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        DivisionChild::Slot(builder.build())
    }
}
impl From<Small> for DivisionChild {
    fn from(child: Small) -> Self {
        DivisionChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for DivisionChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        DivisionChild::Small(builder.build())
    }
}
impl From<Span> for DivisionChild {
    fn from(child: Span) -> Self {
        DivisionChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for DivisionChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        DivisionChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for DivisionChild {
    fn from(child: StrikeThrough) -> Self {
        DivisionChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for DivisionChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        DivisionChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for DivisionChild {
    fn from(child: Strong) -> Self {
        DivisionChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for DivisionChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        DivisionChild::Strong(builder.build())
    }
}
impl From<SubScript> for DivisionChild {
    fn from(child: SubScript) -> Self {
        DivisionChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for DivisionChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        DivisionChild::SubScript(builder.build())
    }
}
impl From<SupScript> for DivisionChild {
    fn from(child: SupScript) -> Self {
        DivisionChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for DivisionChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        DivisionChild::SupScript(builder.build())
    }
}
impl From<Table> for DivisionChild {
    fn from(child: Table) -> Self {
        DivisionChild::Table(child)
    }
}
impl From<builders::TableBuilder> for DivisionChild {
    fn from(builder: builders::TableBuilder) -> Self {
        DivisionChild::Table(builder.build())
    }
}
impl From<Template> for DivisionChild {
    fn from(child: Template) -> Self {
        DivisionChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DivisionChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DivisionChild::Template(builder.build())
    }
}
impl From<TextArea> for DivisionChild {
    fn from(child: TextArea) -> Self {
        DivisionChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for DivisionChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        DivisionChild::TextArea(builder.build())
    }
}
impl From<Time> for DivisionChild {
    fn from(child: Time) -> Self {
        DivisionChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for DivisionChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        DivisionChild::Time(builder.build())
    }
}
impl From<Underline> for DivisionChild {
    fn from(child: Underline) -> Self {
        DivisionChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for DivisionChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        DivisionChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for DivisionChild {
    fn from(child: UnorderedList) -> Self {
        DivisionChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for DivisionChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        DivisionChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for DivisionChild {
    fn from(child: Variable) -> Self {
        DivisionChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for DivisionChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        DivisionChild::Variable(builder.build())
    }
}
impl From<Video> for DivisionChild {
    fn from(child: Video) -> Self {
        DivisionChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for DivisionChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        DivisionChild::Video(builder.build())
    }
}
impl From<WordBreak> for DivisionChild {
    fn from(child: WordBreak) -> Self {
        DivisionChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for DivisionChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        DivisionChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for DivisionChild {
    fn from(s: &'static str) -> Self {
        DivisionChild::Text(s.into())
    }
}
impl From<String> for DivisionChild {
    fn from(s: String) -> Self {
        DivisionChild::Text(s.into())
    }
}
impl From<CowStr> for DivisionChild {
    fn from(s: CowStr) -> Self {
        DivisionChild::Text(s)
    }
}
impl std::fmt::Debug for DivisionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DivisionChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Address(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Article(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Button(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Code(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Data(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Details(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Division(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Form(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Header(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Image(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Input(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Label(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Link(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Main(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Map(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Object(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Output(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Search(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Section(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Select(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Small(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Span(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Table(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Template(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Time(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Video(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            DivisionChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for DivisionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DivisionChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Address(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Article(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Aside(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Audio(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DivisionChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DivisionChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Bold(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Button(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Cite(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Code(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Custom(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Data(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::DataList(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Definition(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Details(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Division(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Embed(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Figure(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Footer(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Form(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Header(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Image(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Input(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Italic(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Label(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Link(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Main(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Map(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Mark(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Menu(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Meter(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Object(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Output(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Picture(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Progress(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Quote(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Sample(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Script(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Search(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Section(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Select(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Slot(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Small(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Span(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Strong(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Table(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Template(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Time(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Underline(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Variable(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Video(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            DivisionChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
