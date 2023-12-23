// 🤖 This file is generated!

use crate::*;
/// The `<section>` element's children.
#[derive(Clone)]
pub enum SectionChild {
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
impl From<Abbreviation> for SectionChild {
    fn from(child: Abbreviation) -> Self {
        SectionChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for SectionChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        SectionChild::Abbreviation(builder.build())
    }
}
impl From<Address> for SectionChild {
    fn from(child: Address) -> Self {
        SectionChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for SectionChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        SectionChild::Address(builder.build())
    }
}
impl From<Anchor> for SectionChild {
    fn from(child: Anchor) -> Self {
        SectionChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for SectionChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        SectionChild::Anchor(builder.build())
    }
}
impl From<Article> for SectionChild {
    fn from(child: Article) -> Self {
        SectionChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for SectionChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        SectionChild::Article(builder.build())
    }
}
impl From<Aside> for SectionChild {
    fn from(child: Aside) -> Self {
        SectionChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for SectionChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        SectionChild::Aside(builder.build())
    }
}
impl From<Audio> for SectionChild {
    fn from(child: Audio) -> Self {
        SectionChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for SectionChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        SectionChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for SectionChild {
    fn from(child: BidirectionalIsolate) -> Self {
        SectionChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for SectionChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        SectionChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for SectionChild {
    fn from(child: BidirectionalOverride) -> Self {
        SectionChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for SectionChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        SectionChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for SectionChild {
    fn from(child: BlockQuote) -> Self {
        SectionChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for SectionChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        SectionChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for SectionChild {
    fn from(child: Bold) -> Self {
        SectionChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for SectionChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        SectionChild::Bold(builder.build())
    }
}
impl From<Button> for SectionChild {
    fn from(child: Button) -> Self {
        SectionChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for SectionChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        SectionChild::Button(builder.build())
    }
}
impl From<Canvas> for SectionChild {
    fn from(child: Canvas) -> Self {
        SectionChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for SectionChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        SectionChild::Canvas(builder.build())
    }
}
impl From<Cite> for SectionChild {
    fn from(child: Cite) -> Self {
        SectionChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for SectionChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        SectionChild::Cite(builder.build())
    }
}
impl From<Code> for SectionChild {
    fn from(child: Code) -> Self {
        SectionChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for SectionChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        SectionChild::Code(builder.build())
    }
}
impl From<Custom> for SectionChild {
    fn from(child: Custom) -> Self {
        SectionChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for SectionChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        SectionChild::Custom(builder.build())
    }
}
impl From<Data> for SectionChild {
    fn from(child: Data) -> Self {
        SectionChild::Data(child)
    }
}
impl From<builders::DataBuilder> for SectionChild {
    fn from(builder: builders::DataBuilder) -> Self {
        SectionChild::Data(builder.build())
    }
}
impl From<DataList> for SectionChild {
    fn from(child: DataList) -> Self {
        SectionChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for SectionChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        SectionChild::DataList(builder.build())
    }
}
impl From<Definition> for SectionChild {
    fn from(child: Definition) -> Self {
        SectionChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for SectionChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        SectionChild::Definition(builder.build())
    }
}
impl From<Deleted> for SectionChild {
    fn from(child: Deleted) -> Self {
        SectionChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for SectionChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        SectionChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for SectionChild {
    fn from(child: DescriptionList) -> Self {
        SectionChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for SectionChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        SectionChild::DescriptionList(builder.build())
    }
}
impl From<Details> for SectionChild {
    fn from(child: Details) -> Self {
        SectionChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for SectionChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        SectionChild::Details(builder.build())
    }
}
impl From<Dialog> for SectionChild {
    fn from(child: Dialog) -> Self {
        SectionChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for SectionChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        SectionChild::Dialog(builder.build())
    }
}
impl From<Division> for SectionChild {
    fn from(child: Division) -> Self {
        SectionChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for SectionChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        SectionChild::Division(builder.build())
    }
}
impl From<Embed> for SectionChild {
    fn from(child: Embed) -> Self {
        SectionChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for SectionChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        SectionChild::Embed(builder.build())
    }
}
impl From<Emphasis> for SectionChild {
    fn from(child: Emphasis) -> Self {
        SectionChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for SectionChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        SectionChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for SectionChild {
    fn from(child: FieldSet) -> Self {
        SectionChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for SectionChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        SectionChild::FieldSet(builder.build())
    }
}
impl From<Figure> for SectionChild {
    fn from(child: Figure) -> Self {
        SectionChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for SectionChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        SectionChild::Figure(builder.build())
    }
}
impl From<Footer> for SectionChild {
    fn from(child: Footer) -> Self {
        SectionChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for SectionChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        SectionChild::Footer(builder.build())
    }
}
impl From<Form> for SectionChild {
    fn from(child: Form) -> Self {
        SectionChild::Form(child)
    }
}
impl From<builders::FormBuilder> for SectionChild {
    fn from(builder: builders::FormBuilder) -> Self {
        SectionChild::Form(builder.build())
    }
}
impl From<Header> for SectionChild {
    fn from(child: Header) -> Self {
        SectionChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for SectionChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        SectionChild::Header(builder.build())
    }
}
impl From<Heading1> for SectionChild {
    fn from(child: Heading1) -> Self {
        SectionChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for SectionChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        SectionChild::Heading1(builder.build())
    }
}
impl From<Heading2> for SectionChild {
    fn from(child: Heading2) -> Self {
        SectionChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for SectionChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        SectionChild::Heading2(builder.build())
    }
}
impl From<Heading3> for SectionChild {
    fn from(child: Heading3) -> Self {
        SectionChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for SectionChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        SectionChild::Heading3(builder.build())
    }
}
impl From<Heading4> for SectionChild {
    fn from(child: Heading4) -> Self {
        SectionChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for SectionChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        SectionChild::Heading4(builder.build())
    }
}
impl From<Heading5> for SectionChild {
    fn from(child: Heading5) -> Self {
        SectionChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for SectionChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        SectionChild::Heading5(builder.build())
    }
}
impl From<Heading6> for SectionChild {
    fn from(child: Heading6) -> Self {
        SectionChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for SectionChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        SectionChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for SectionChild {
    fn from(child: HeadingGroup) -> Self {
        SectionChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for SectionChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        SectionChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for SectionChild {
    fn from(child: HorizontalRule) -> Self {
        SectionChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for SectionChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        SectionChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for SectionChild {
    fn from(child: Image) -> Self {
        SectionChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for SectionChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        SectionChild::Image(builder.build())
    }
}
impl From<InlineFrame> for SectionChild {
    fn from(child: InlineFrame) -> Self {
        SectionChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for SectionChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        SectionChild::InlineFrame(builder.build())
    }
}
impl From<Input> for SectionChild {
    fn from(child: Input) -> Self {
        SectionChild::Input(child)
    }
}
impl From<builders::InputBuilder> for SectionChild {
    fn from(builder: builders::InputBuilder) -> Self {
        SectionChild::Input(builder.build())
    }
}
impl From<Inserted> for SectionChild {
    fn from(child: Inserted) -> Self {
        SectionChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for SectionChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        SectionChild::Inserted(builder.build())
    }
}
impl From<Italic> for SectionChild {
    fn from(child: Italic) -> Self {
        SectionChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for SectionChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        SectionChild::Italic(builder.build())
    }
}
impl From<Keyboard> for SectionChild {
    fn from(child: Keyboard) -> Self {
        SectionChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for SectionChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        SectionChild::Keyboard(builder.build())
    }
}
impl From<Label> for SectionChild {
    fn from(child: Label) -> Self {
        SectionChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for SectionChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        SectionChild::Label(builder.build())
    }
}
impl From<LineBreak> for SectionChild {
    fn from(child: LineBreak) -> Self {
        SectionChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for SectionChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        SectionChild::LineBreak(builder.build())
    }
}
impl From<Link> for SectionChild {
    fn from(child: Link) -> Self {
        SectionChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for SectionChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        SectionChild::Link(builder.build())
    }
}
impl From<Main> for SectionChild {
    fn from(child: Main) -> Self {
        SectionChild::Main(child)
    }
}
impl From<builders::MainBuilder> for SectionChild {
    fn from(builder: builders::MainBuilder) -> Self {
        SectionChild::Main(builder.build())
    }
}
impl From<Map> for SectionChild {
    fn from(child: Map) -> Self {
        SectionChild::Map(child)
    }
}
impl From<builders::MapBuilder> for SectionChild {
    fn from(builder: builders::MapBuilder) -> Self {
        SectionChild::Map(builder.build())
    }
}
impl From<MapArea> for SectionChild {
    fn from(child: MapArea) -> Self {
        SectionChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for SectionChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        SectionChild::MapArea(builder.build())
    }
}
impl From<Mark> for SectionChild {
    fn from(child: Mark) -> Self {
        SectionChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for SectionChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        SectionChild::Mark(builder.build())
    }
}
impl From<Menu> for SectionChild {
    fn from(child: Menu) -> Self {
        SectionChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for SectionChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        SectionChild::Menu(builder.build())
    }
}
impl From<Metadata> for SectionChild {
    fn from(child: Metadata) -> Self {
        SectionChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for SectionChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        SectionChild::Metadata(builder.build())
    }
}
impl From<Meter> for SectionChild {
    fn from(child: Meter) -> Self {
        SectionChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for SectionChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        SectionChild::Meter(builder.build())
    }
}
impl From<Navigation> for SectionChild {
    fn from(child: Navigation) -> Self {
        SectionChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for SectionChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        SectionChild::Navigation(builder.build())
    }
}
impl From<NoScript> for SectionChild {
    fn from(child: NoScript) -> Self {
        SectionChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for SectionChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        SectionChild::NoScript(builder.build())
    }
}
impl From<Object> for SectionChild {
    fn from(child: Object) -> Self {
        SectionChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for SectionChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        SectionChild::Object(builder.build())
    }
}
impl From<OrderedList> for SectionChild {
    fn from(child: OrderedList) -> Self {
        SectionChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for SectionChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        SectionChild::OrderedList(builder.build())
    }
}
impl From<Output> for SectionChild {
    fn from(child: Output) -> Self {
        SectionChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for SectionChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        SectionChild::Output(builder.build())
    }
}
impl From<Paragraph> for SectionChild {
    fn from(child: Paragraph) -> Self {
        SectionChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for SectionChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        SectionChild::Paragraph(builder.build())
    }
}
impl From<Picture> for SectionChild {
    fn from(child: Picture) -> Self {
        SectionChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for SectionChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        SectionChild::Picture(builder.build())
    }
}
impl From<Preformatted> for SectionChild {
    fn from(child: Preformatted) -> Self {
        SectionChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for SectionChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        SectionChild::Preformatted(builder.build())
    }
}
impl From<Progress> for SectionChild {
    fn from(child: Progress) -> Self {
        SectionChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for SectionChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        SectionChild::Progress(builder.build())
    }
}
impl From<Quote> for SectionChild {
    fn from(child: Quote) -> Self {
        SectionChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for SectionChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        SectionChild::Quote(builder.build())
    }
}
impl From<Ruby> for SectionChild {
    fn from(child: Ruby) -> Self {
        SectionChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for SectionChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        SectionChild::Ruby(builder.build())
    }
}
impl From<Sample> for SectionChild {
    fn from(child: Sample) -> Self {
        SectionChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for SectionChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        SectionChild::Sample(builder.build())
    }
}
impl From<Script> for SectionChild {
    fn from(child: Script) -> Self {
        SectionChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SectionChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SectionChild::Script(builder.build())
    }
}
impl From<Search> for SectionChild {
    fn from(child: Search) -> Self {
        SectionChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for SectionChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        SectionChild::Search(builder.build())
    }
}
impl From<Section> for SectionChild {
    fn from(child: Section) -> Self {
        SectionChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for SectionChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        SectionChild::Section(builder.build())
    }
}
impl From<Select> for SectionChild {
    fn from(child: Select) -> Self {
        SectionChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for SectionChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        SectionChild::Select(builder.build())
    }
}
impl From<Slot> for SectionChild {
    fn from(child: Slot) -> Self {
        SectionChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for SectionChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        SectionChild::Slot(builder.build())
    }
}
impl From<Small> for SectionChild {
    fn from(child: Small) -> Self {
        SectionChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for SectionChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        SectionChild::Small(builder.build())
    }
}
impl From<Span> for SectionChild {
    fn from(child: Span) -> Self {
        SectionChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for SectionChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        SectionChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for SectionChild {
    fn from(child: StrikeThrough) -> Self {
        SectionChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for SectionChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        SectionChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for SectionChild {
    fn from(child: Strong) -> Self {
        SectionChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for SectionChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        SectionChild::Strong(builder.build())
    }
}
impl From<SubScript> for SectionChild {
    fn from(child: SubScript) -> Self {
        SectionChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for SectionChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        SectionChild::SubScript(builder.build())
    }
}
impl From<SupScript> for SectionChild {
    fn from(child: SupScript) -> Self {
        SectionChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for SectionChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        SectionChild::SupScript(builder.build())
    }
}
impl From<Table> for SectionChild {
    fn from(child: Table) -> Self {
        SectionChild::Table(child)
    }
}
impl From<builders::TableBuilder> for SectionChild {
    fn from(builder: builders::TableBuilder) -> Self {
        SectionChild::Table(builder.build())
    }
}
impl From<Template> for SectionChild {
    fn from(child: Template) -> Self {
        SectionChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SectionChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SectionChild::Template(builder.build())
    }
}
impl From<TextArea> for SectionChild {
    fn from(child: TextArea) -> Self {
        SectionChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for SectionChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        SectionChild::TextArea(builder.build())
    }
}
impl From<Time> for SectionChild {
    fn from(child: Time) -> Self {
        SectionChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for SectionChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        SectionChild::Time(builder.build())
    }
}
impl From<Underline> for SectionChild {
    fn from(child: Underline) -> Self {
        SectionChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for SectionChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        SectionChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for SectionChild {
    fn from(child: UnorderedList) -> Self {
        SectionChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for SectionChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        SectionChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for SectionChild {
    fn from(child: Variable) -> Self {
        SectionChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for SectionChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        SectionChild::Variable(builder.build())
    }
}
impl From<Video> for SectionChild {
    fn from(child: Video) -> Self {
        SectionChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for SectionChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        SectionChild::Video(builder.build())
    }
}
impl From<WordBreak> for SectionChild {
    fn from(child: WordBreak) -> Self {
        SectionChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for SectionChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        SectionChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for SectionChild {
    fn from(s: &'static str) -> Self {
        SectionChild::Text(s.into())
    }
}
impl From<String> for SectionChild {
    fn from(s: String) -> Self {
        SectionChild::Text(s.into())
    }
}
impl From<CowStr> for SectionChild {
    fn from(s: CowStr) -> Self {
        SectionChild::Text(s)
    }
}
impl std::fmt::Debug for SectionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SectionChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Address(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Article(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Button(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Code(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Data(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Details(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Division(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Form(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Header(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Image(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Input(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Label(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Link(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Main(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Map(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Object(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Output(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Search(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Section(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Select(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Small(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Span(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Table(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Template(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Time(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Video(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            SectionChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for SectionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SectionChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Address(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Article(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Aside(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Audio(child) => std::fmt::Display::fmt(child, f),
            SectionChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            SectionChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            SectionChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Bold(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Button(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Cite(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Code(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Custom(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Data(child) => std::fmt::Display::fmt(child, f),
            SectionChild::DataList(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Definition(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            SectionChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Details(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Division(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Embed(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            SectionChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Figure(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Footer(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Form(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Header(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            SectionChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            SectionChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Image(child) => std::fmt::Display::fmt(child, f),
            SectionChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Input(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Italic(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Label(child) => std::fmt::Display::fmt(child, f),
            SectionChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Link(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Main(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Map(child) => std::fmt::Display::fmt(child, f),
            SectionChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Mark(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Menu(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Meter(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            SectionChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Object(child) => std::fmt::Display::fmt(child, f),
            SectionChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Output(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Picture(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Progress(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Quote(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Sample(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Script(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Search(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Section(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Select(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Slot(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Small(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Span(child) => std::fmt::Display::fmt(child, f),
            SectionChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Strong(child) => std::fmt::Display::fmt(child, f),
            SectionChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            SectionChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Table(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Template(child) => std::fmt::Display::fmt(child, f),
            SectionChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Time(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Underline(child) => std::fmt::Display::fmt(child, f),
            SectionChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Variable(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Video(child) => std::fmt::Display::fmt(child, f),
            SectionChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            SectionChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
