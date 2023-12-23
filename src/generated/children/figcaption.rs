// 🤖 This file is generated!

use crate::*;
/// The `<figcaption>` element's children.
#[derive(Clone)]
pub enum FigureCaptionChild {
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
impl From<Abbreviation> for FigureCaptionChild {
    fn from(child: Abbreviation) -> Self {
        FigureCaptionChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for FigureCaptionChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        FigureCaptionChild::Abbreviation(builder.build())
    }
}
impl From<Address> for FigureCaptionChild {
    fn from(child: Address) -> Self {
        FigureCaptionChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for FigureCaptionChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        FigureCaptionChild::Address(builder.build())
    }
}
impl From<Anchor> for FigureCaptionChild {
    fn from(child: Anchor) -> Self {
        FigureCaptionChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for FigureCaptionChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        FigureCaptionChild::Anchor(builder.build())
    }
}
impl From<Article> for FigureCaptionChild {
    fn from(child: Article) -> Self {
        FigureCaptionChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for FigureCaptionChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        FigureCaptionChild::Article(builder.build())
    }
}
impl From<Aside> for FigureCaptionChild {
    fn from(child: Aside) -> Self {
        FigureCaptionChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for FigureCaptionChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        FigureCaptionChild::Aside(builder.build())
    }
}
impl From<Audio> for FigureCaptionChild {
    fn from(child: Audio) -> Self {
        FigureCaptionChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for FigureCaptionChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        FigureCaptionChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for FigureCaptionChild {
    fn from(child: BidirectionalIsolate) -> Self {
        FigureCaptionChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for FigureCaptionChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        FigureCaptionChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for FigureCaptionChild {
    fn from(child: BidirectionalOverride) -> Self {
        FigureCaptionChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for FigureCaptionChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        FigureCaptionChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for FigureCaptionChild {
    fn from(child: BlockQuote) -> Self {
        FigureCaptionChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for FigureCaptionChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        FigureCaptionChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for FigureCaptionChild {
    fn from(child: Bold) -> Self {
        FigureCaptionChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for FigureCaptionChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        FigureCaptionChild::Bold(builder.build())
    }
}
impl From<Button> for FigureCaptionChild {
    fn from(child: Button) -> Self {
        FigureCaptionChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for FigureCaptionChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        FigureCaptionChild::Button(builder.build())
    }
}
impl From<Canvas> for FigureCaptionChild {
    fn from(child: Canvas) -> Self {
        FigureCaptionChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for FigureCaptionChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        FigureCaptionChild::Canvas(builder.build())
    }
}
impl From<Cite> for FigureCaptionChild {
    fn from(child: Cite) -> Self {
        FigureCaptionChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for FigureCaptionChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        FigureCaptionChild::Cite(builder.build())
    }
}
impl From<Code> for FigureCaptionChild {
    fn from(child: Code) -> Self {
        FigureCaptionChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for FigureCaptionChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        FigureCaptionChild::Code(builder.build())
    }
}
impl From<Custom> for FigureCaptionChild {
    fn from(child: Custom) -> Self {
        FigureCaptionChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for FigureCaptionChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        FigureCaptionChild::Custom(builder.build())
    }
}
impl From<Data> for FigureCaptionChild {
    fn from(child: Data) -> Self {
        FigureCaptionChild::Data(child)
    }
}
impl From<builders::DataBuilder> for FigureCaptionChild {
    fn from(builder: builders::DataBuilder) -> Self {
        FigureCaptionChild::Data(builder.build())
    }
}
impl From<DataList> for FigureCaptionChild {
    fn from(child: DataList) -> Self {
        FigureCaptionChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for FigureCaptionChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        FigureCaptionChild::DataList(builder.build())
    }
}
impl From<Definition> for FigureCaptionChild {
    fn from(child: Definition) -> Self {
        FigureCaptionChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for FigureCaptionChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        FigureCaptionChild::Definition(builder.build())
    }
}
impl From<Deleted> for FigureCaptionChild {
    fn from(child: Deleted) -> Self {
        FigureCaptionChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for FigureCaptionChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        FigureCaptionChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for FigureCaptionChild {
    fn from(child: DescriptionList) -> Self {
        FigureCaptionChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for FigureCaptionChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        FigureCaptionChild::DescriptionList(builder.build())
    }
}
impl From<Details> for FigureCaptionChild {
    fn from(child: Details) -> Self {
        FigureCaptionChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for FigureCaptionChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        FigureCaptionChild::Details(builder.build())
    }
}
impl From<Dialog> for FigureCaptionChild {
    fn from(child: Dialog) -> Self {
        FigureCaptionChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for FigureCaptionChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        FigureCaptionChild::Dialog(builder.build())
    }
}
impl From<Division> for FigureCaptionChild {
    fn from(child: Division) -> Self {
        FigureCaptionChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for FigureCaptionChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        FigureCaptionChild::Division(builder.build())
    }
}
impl From<Embed> for FigureCaptionChild {
    fn from(child: Embed) -> Self {
        FigureCaptionChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for FigureCaptionChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        FigureCaptionChild::Embed(builder.build())
    }
}
impl From<Emphasis> for FigureCaptionChild {
    fn from(child: Emphasis) -> Self {
        FigureCaptionChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for FigureCaptionChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        FigureCaptionChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for FigureCaptionChild {
    fn from(child: FieldSet) -> Self {
        FigureCaptionChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for FigureCaptionChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        FigureCaptionChild::FieldSet(builder.build())
    }
}
impl From<Figure> for FigureCaptionChild {
    fn from(child: Figure) -> Self {
        FigureCaptionChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for FigureCaptionChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        FigureCaptionChild::Figure(builder.build())
    }
}
impl From<Footer> for FigureCaptionChild {
    fn from(child: Footer) -> Self {
        FigureCaptionChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for FigureCaptionChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        FigureCaptionChild::Footer(builder.build())
    }
}
impl From<Form> for FigureCaptionChild {
    fn from(child: Form) -> Self {
        FigureCaptionChild::Form(child)
    }
}
impl From<builders::FormBuilder> for FigureCaptionChild {
    fn from(builder: builders::FormBuilder) -> Self {
        FigureCaptionChild::Form(builder.build())
    }
}
impl From<Header> for FigureCaptionChild {
    fn from(child: Header) -> Self {
        FigureCaptionChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for FigureCaptionChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        FigureCaptionChild::Header(builder.build())
    }
}
impl From<Heading1> for FigureCaptionChild {
    fn from(child: Heading1) -> Self {
        FigureCaptionChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for FigureCaptionChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        FigureCaptionChild::Heading1(builder.build())
    }
}
impl From<Heading2> for FigureCaptionChild {
    fn from(child: Heading2) -> Self {
        FigureCaptionChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for FigureCaptionChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        FigureCaptionChild::Heading2(builder.build())
    }
}
impl From<Heading3> for FigureCaptionChild {
    fn from(child: Heading3) -> Self {
        FigureCaptionChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for FigureCaptionChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        FigureCaptionChild::Heading3(builder.build())
    }
}
impl From<Heading4> for FigureCaptionChild {
    fn from(child: Heading4) -> Self {
        FigureCaptionChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for FigureCaptionChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        FigureCaptionChild::Heading4(builder.build())
    }
}
impl From<Heading5> for FigureCaptionChild {
    fn from(child: Heading5) -> Self {
        FigureCaptionChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for FigureCaptionChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        FigureCaptionChild::Heading5(builder.build())
    }
}
impl From<Heading6> for FigureCaptionChild {
    fn from(child: Heading6) -> Self {
        FigureCaptionChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for FigureCaptionChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        FigureCaptionChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for FigureCaptionChild {
    fn from(child: HeadingGroup) -> Self {
        FigureCaptionChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for FigureCaptionChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        FigureCaptionChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for FigureCaptionChild {
    fn from(child: HorizontalRule) -> Self {
        FigureCaptionChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for FigureCaptionChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        FigureCaptionChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for FigureCaptionChild {
    fn from(child: Image) -> Self {
        FigureCaptionChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for FigureCaptionChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        FigureCaptionChild::Image(builder.build())
    }
}
impl From<InlineFrame> for FigureCaptionChild {
    fn from(child: InlineFrame) -> Self {
        FigureCaptionChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for FigureCaptionChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        FigureCaptionChild::InlineFrame(builder.build())
    }
}
impl From<Input> for FigureCaptionChild {
    fn from(child: Input) -> Self {
        FigureCaptionChild::Input(child)
    }
}
impl From<builders::InputBuilder> for FigureCaptionChild {
    fn from(builder: builders::InputBuilder) -> Self {
        FigureCaptionChild::Input(builder.build())
    }
}
impl From<Inserted> for FigureCaptionChild {
    fn from(child: Inserted) -> Self {
        FigureCaptionChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for FigureCaptionChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        FigureCaptionChild::Inserted(builder.build())
    }
}
impl From<Italic> for FigureCaptionChild {
    fn from(child: Italic) -> Self {
        FigureCaptionChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for FigureCaptionChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        FigureCaptionChild::Italic(builder.build())
    }
}
impl From<Keyboard> for FigureCaptionChild {
    fn from(child: Keyboard) -> Self {
        FigureCaptionChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for FigureCaptionChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        FigureCaptionChild::Keyboard(builder.build())
    }
}
impl From<Label> for FigureCaptionChild {
    fn from(child: Label) -> Self {
        FigureCaptionChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for FigureCaptionChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        FigureCaptionChild::Label(builder.build())
    }
}
impl From<LineBreak> for FigureCaptionChild {
    fn from(child: LineBreak) -> Self {
        FigureCaptionChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for FigureCaptionChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        FigureCaptionChild::LineBreak(builder.build())
    }
}
impl From<Link> for FigureCaptionChild {
    fn from(child: Link) -> Self {
        FigureCaptionChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for FigureCaptionChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        FigureCaptionChild::Link(builder.build())
    }
}
impl From<Main> for FigureCaptionChild {
    fn from(child: Main) -> Self {
        FigureCaptionChild::Main(child)
    }
}
impl From<builders::MainBuilder> for FigureCaptionChild {
    fn from(builder: builders::MainBuilder) -> Self {
        FigureCaptionChild::Main(builder.build())
    }
}
impl From<Map> for FigureCaptionChild {
    fn from(child: Map) -> Self {
        FigureCaptionChild::Map(child)
    }
}
impl From<builders::MapBuilder> for FigureCaptionChild {
    fn from(builder: builders::MapBuilder) -> Self {
        FigureCaptionChild::Map(builder.build())
    }
}
impl From<MapArea> for FigureCaptionChild {
    fn from(child: MapArea) -> Self {
        FigureCaptionChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for FigureCaptionChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        FigureCaptionChild::MapArea(builder.build())
    }
}
impl From<Mark> for FigureCaptionChild {
    fn from(child: Mark) -> Self {
        FigureCaptionChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for FigureCaptionChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        FigureCaptionChild::Mark(builder.build())
    }
}
impl From<Menu> for FigureCaptionChild {
    fn from(child: Menu) -> Self {
        FigureCaptionChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for FigureCaptionChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        FigureCaptionChild::Menu(builder.build())
    }
}
impl From<Metadata> for FigureCaptionChild {
    fn from(child: Metadata) -> Self {
        FigureCaptionChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for FigureCaptionChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        FigureCaptionChild::Metadata(builder.build())
    }
}
impl From<Meter> for FigureCaptionChild {
    fn from(child: Meter) -> Self {
        FigureCaptionChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for FigureCaptionChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        FigureCaptionChild::Meter(builder.build())
    }
}
impl From<Navigation> for FigureCaptionChild {
    fn from(child: Navigation) -> Self {
        FigureCaptionChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for FigureCaptionChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        FigureCaptionChild::Navigation(builder.build())
    }
}
impl From<NoScript> for FigureCaptionChild {
    fn from(child: NoScript) -> Self {
        FigureCaptionChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for FigureCaptionChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        FigureCaptionChild::NoScript(builder.build())
    }
}
impl From<Object> for FigureCaptionChild {
    fn from(child: Object) -> Self {
        FigureCaptionChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for FigureCaptionChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        FigureCaptionChild::Object(builder.build())
    }
}
impl From<OrderedList> for FigureCaptionChild {
    fn from(child: OrderedList) -> Self {
        FigureCaptionChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for FigureCaptionChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        FigureCaptionChild::OrderedList(builder.build())
    }
}
impl From<Output> for FigureCaptionChild {
    fn from(child: Output) -> Self {
        FigureCaptionChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for FigureCaptionChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        FigureCaptionChild::Output(builder.build())
    }
}
impl From<Paragraph> for FigureCaptionChild {
    fn from(child: Paragraph) -> Self {
        FigureCaptionChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for FigureCaptionChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        FigureCaptionChild::Paragraph(builder.build())
    }
}
impl From<Picture> for FigureCaptionChild {
    fn from(child: Picture) -> Self {
        FigureCaptionChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for FigureCaptionChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        FigureCaptionChild::Picture(builder.build())
    }
}
impl From<Preformatted> for FigureCaptionChild {
    fn from(child: Preformatted) -> Self {
        FigureCaptionChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for FigureCaptionChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        FigureCaptionChild::Preformatted(builder.build())
    }
}
impl From<Progress> for FigureCaptionChild {
    fn from(child: Progress) -> Self {
        FigureCaptionChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for FigureCaptionChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        FigureCaptionChild::Progress(builder.build())
    }
}
impl From<Quote> for FigureCaptionChild {
    fn from(child: Quote) -> Self {
        FigureCaptionChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for FigureCaptionChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        FigureCaptionChild::Quote(builder.build())
    }
}
impl From<Ruby> for FigureCaptionChild {
    fn from(child: Ruby) -> Self {
        FigureCaptionChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for FigureCaptionChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        FigureCaptionChild::Ruby(builder.build())
    }
}
impl From<Sample> for FigureCaptionChild {
    fn from(child: Sample) -> Self {
        FigureCaptionChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for FigureCaptionChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        FigureCaptionChild::Sample(builder.build())
    }
}
impl From<Script> for FigureCaptionChild {
    fn from(child: Script) -> Self {
        FigureCaptionChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for FigureCaptionChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        FigureCaptionChild::Script(builder.build())
    }
}
impl From<Search> for FigureCaptionChild {
    fn from(child: Search) -> Self {
        FigureCaptionChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for FigureCaptionChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        FigureCaptionChild::Search(builder.build())
    }
}
impl From<Section> for FigureCaptionChild {
    fn from(child: Section) -> Self {
        FigureCaptionChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for FigureCaptionChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        FigureCaptionChild::Section(builder.build())
    }
}
impl From<Select> for FigureCaptionChild {
    fn from(child: Select) -> Self {
        FigureCaptionChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for FigureCaptionChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        FigureCaptionChild::Select(builder.build())
    }
}
impl From<Slot> for FigureCaptionChild {
    fn from(child: Slot) -> Self {
        FigureCaptionChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for FigureCaptionChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        FigureCaptionChild::Slot(builder.build())
    }
}
impl From<Small> for FigureCaptionChild {
    fn from(child: Small) -> Self {
        FigureCaptionChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for FigureCaptionChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        FigureCaptionChild::Small(builder.build())
    }
}
impl From<Span> for FigureCaptionChild {
    fn from(child: Span) -> Self {
        FigureCaptionChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for FigureCaptionChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        FigureCaptionChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for FigureCaptionChild {
    fn from(child: StrikeThrough) -> Self {
        FigureCaptionChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for FigureCaptionChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        FigureCaptionChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for FigureCaptionChild {
    fn from(child: Strong) -> Self {
        FigureCaptionChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for FigureCaptionChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        FigureCaptionChild::Strong(builder.build())
    }
}
impl From<SubScript> for FigureCaptionChild {
    fn from(child: SubScript) -> Self {
        FigureCaptionChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for FigureCaptionChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        FigureCaptionChild::SubScript(builder.build())
    }
}
impl From<SupScript> for FigureCaptionChild {
    fn from(child: SupScript) -> Self {
        FigureCaptionChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for FigureCaptionChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        FigureCaptionChild::SupScript(builder.build())
    }
}
impl From<Table> for FigureCaptionChild {
    fn from(child: Table) -> Self {
        FigureCaptionChild::Table(child)
    }
}
impl From<builders::TableBuilder> for FigureCaptionChild {
    fn from(builder: builders::TableBuilder) -> Self {
        FigureCaptionChild::Table(builder.build())
    }
}
impl From<Template> for FigureCaptionChild {
    fn from(child: Template) -> Self {
        FigureCaptionChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for FigureCaptionChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        FigureCaptionChild::Template(builder.build())
    }
}
impl From<TextArea> for FigureCaptionChild {
    fn from(child: TextArea) -> Self {
        FigureCaptionChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for FigureCaptionChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        FigureCaptionChild::TextArea(builder.build())
    }
}
impl From<Time> for FigureCaptionChild {
    fn from(child: Time) -> Self {
        FigureCaptionChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for FigureCaptionChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        FigureCaptionChild::Time(builder.build())
    }
}
impl From<Underline> for FigureCaptionChild {
    fn from(child: Underline) -> Self {
        FigureCaptionChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for FigureCaptionChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        FigureCaptionChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for FigureCaptionChild {
    fn from(child: UnorderedList) -> Self {
        FigureCaptionChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for FigureCaptionChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        FigureCaptionChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for FigureCaptionChild {
    fn from(child: Variable) -> Self {
        FigureCaptionChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for FigureCaptionChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        FigureCaptionChild::Variable(builder.build())
    }
}
impl From<Video> for FigureCaptionChild {
    fn from(child: Video) -> Self {
        FigureCaptionChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for FigureCaptionChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        FigureCaptionChild::Video(builder.build())
    }
}
impl From<WordBreak> for FigureCaptionChild {
    fn from(child: WordBreak) -> Self {
        FigureCaptionChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for FigureCaptionChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        FigureCaptionChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for FigureCaptionChild {
    fn from(s: &'static str) -> Self {
        FigureCaptionChild::Text(s.into())
    }
}
impl From<String> for FigureCaptionChild {
    fn from(s: String) -> Self {
        FigureCaptionChild::Text(s.into())
    }
}
impl From<CowStr> for FigureCaptionChild {
    fn from(s: CowStr) -> Self {
        FigureCaptionChild::Text(s)
    }
}
impl std::fmt::Debug for FigureCaptionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FigureCaptionChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Address(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Article(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            FigureCaptionChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            FigureCaptionChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Button(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Code(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Data(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Details(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Division(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Form(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Header(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Image(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Input(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Label(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Link(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Main(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Map(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Object(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Output(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Script(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Search(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Section(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Select(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Small(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Span(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Table(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Template(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Time(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Video(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            FigureCaptionChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for FigureCaptionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FigureCaptionChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Address(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Article(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Aside(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Audio(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            FigureCaptionChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            FigureCaptionChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Bold(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Button(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Cite(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Code(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Custom(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Data(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::DataList(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Definition(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::DescriptionList(child) => {
                std::fmt::Display::fmt(child, f)
            }
            FigureCaptionChild::Details(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Division(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Embed(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Figure(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Footer(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Form(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Header(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Image(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Input(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Italic(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Label(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Link(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Main(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Map(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Mark(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Menu(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Meter(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Object(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Output(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Picture(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Progress(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Quote(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Sample(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Script(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Search(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Section(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Select(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Slot(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Small(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Span(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Strong(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Table(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Template(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Time(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Underline(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Variable(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Video(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            FigureCaptionChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
