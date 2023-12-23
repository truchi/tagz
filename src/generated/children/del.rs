// 🤖 This file is generated!

use crate::*;
/// The `<del>` element's children.
#[derive(Clone)]
pub enum DeletedChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    Base(Base),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
    Body(Body),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Caption(Caption),
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
    FieldSetLegend(FieldSetLegend),
    Figure(Figure),
    FigureCaption(FigureCaption),
    Footer(Footer),
    Form(Form),
    Head(Head),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Html(Html),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    ListItem(ListItem),
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
    Option(Option),
    OptionGroup(OptionGroup),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    RubyParenthesis(RubyParenthesis),
    RubyText(RubyText),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Source(Source),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    Style(Style),
    SubScript(SubScript),
    Summary(Summary),
    SupScript(SupScript),
    Table(Table),
    TableBody(TableBody),
    TableCell(TableCell),
    TableColumn(TableColumn),
    TableColumnGroup(TableColumnGroup),
    TableFoot(TableFoot),
    TableHead(TableHead),
    TableHeader(TableHeader),
    TableRow(TableRow),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Title(Title),
    Track(Track),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
}
impl From<Abbreviation> for DeletedChild {
    fn from(child: Abbreviation) -> Self {
        DeletedChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for DeletedChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        DeletedChild::Abbreviation(builder.build())
    }
}
impl From<Address> for DeletedChild {
    fn from(child: Address) -> Self {
        DeletedChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for DeletedChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        DeletedChild::Address(builder.build())
    }
}
impl From<Anchor> for DeletedChild {
    fn from(child: Anchor) -> Self {
        DeletedChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for DeletedChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        DeletedChild::Anchor(builder.build())
    }
}
impl From<Article> for DeletedChild {
    fn from(child: Article) -> Self {
        DeletedChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for DeletedChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        DeletedChild::Article(builder.build())
    }
}
impl From<Aside> for DeletedChild {
    fn from(child: Aside) -> Self {
        DeletedChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for DeletedChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        DeletedChild::Aside(builder.build())
    }
}
impl From<Audio> for DeletedChild {
    fn from(child: Audio) -> Self {
        DeletedChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for DeletedChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        DeletedChild::Audio(builder.build())
    }
}
impl From<Base> for DeletedChild {
    fn from(child: Base) -> Self {
        DeletedChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for DeletedChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        DeletedChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for DeletedChild {
    fn from(child: BidirectionalIsolate) -> Self {
        DeletedChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for DeletedChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        DeletedChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for DeletedChild {
    fn from(child: BidirectionalOverride) -> Self {
        DeletedChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for DeletedChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        DeletedChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for DeletedChild {
    fn from(child: BlockQuote) -> Self {
        DeletedChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for DeletedChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        DeletedChild::BlockQuote(builder.build())
    }
}
impl From<Body> for DeletedChild {
    fn from(child: Body) -> Self {
        DeletedChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for DeletedChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        DeletedChild::Body(builder.build())
    }
}
impl From<Bold> for DeletedChild {
    fn from(child: Bold) -> Self {
        DeletedChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for DeletedChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        DeletedChild::Bold(builder.build())
    }
}
impl From<Button> for DeletedChild {
    fn from(child: Button) -> Self {
        DeletedChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for DeletedChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        DeletedChild::Button(builder.build())
    }
}
impl From<Canvas> for DeletedChild {
    fn from(child: Canvas) -> Self {
        DeletedChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for DeletedChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        DeletedChild::Canvas(builder.build())
    }
}
impl From<Caption> for DeletedChild {
    fn from(child: Caption) -> Self {
        DeletedChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for DeletedChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        DeletedChild::Caption(builder.build())
    }
}
impl From<Cite> for DeletedChild {
    fn from(child: Cite) -> Self {
        DeletedChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for DeletedChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        DeletedChild::Cite(builder.build())
    }
}
impl From<Code> for DeletedChild {
    fn from(child: Code) -> Self {
        DeletedChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for DeletedChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        DeletedChild::Code(builder.build())
    }
}
impl From<Custom> for DeletedChild {
    fn from(child: Custom) -> Self {
        DeletedChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for DeletedChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        DeletedChild::Custom(builder.build())
    }
}
impl From<Data> for DeletedChild {
    fn from(child: Data) -> Self {
        DeletedChild::Data(child)
    }
}
impl From<builders::DataBuilder> for DeletedChild {
    fn from(builder: builders::DataBuilder) -> Self {
        DeletedChild::Data(builder.build())
    }
}
impl From<DataList> for DeletedChild {
    fn from(child: DataList) -> Self {
        DeletedChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for DeletedChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        DeletedChild::DataList(builder.build())
    }
}
impl From<Definition> for DeletedChild {
    fn from(child: Definition) -> Self {
        DeletedChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for DeletedChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        DeletedChild::Definition(builder.build())
    }
}
impl From<Deleted> for DeletedChild {
    fn from(child: Deleted) -> Self {
        DeletedChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for DeletedChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        DeletedChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for DeletedChild {
    fn from(child: DescriptionDetails) -> Self {
        DeletedChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for DeletedChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        DeletedChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for DeletedChild {
    fn from(child: DescriptionList) -> Self {
        DeletedChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for DeletedChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        DeletedChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for DeletedChild {
    fn from(child: DescriptionTerm) -> Self {
        DeletedChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for DeletedChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        DeletedChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for DeletedChild {
    fn from(child: Details) -> Self {
        DeletedChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for DeletedChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        DeletedChild::Details(builder.build())
    }
}
impl From<Dialog> for DeletedChild {
    fn from(child: Dialog) -> Self {
        DeletedChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for DeletedChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        DeletedChild::Dialog(builder.build())
    }
}
impl From<Division> for DeletedChild {
    fn from(child: Division) -> Self {
        DeletedChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for DeletedChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        DeletedChild::Division(builder.build())
    }
}
impl From<Embed> for DeletedChild {
    fn from(child: Embed) -> Self {
        DeletedChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for DeletedChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        DeletedChild::Embed(builder.build())
    }
}
impl From<Emphasis> for DeletedChild {
    fn from(child: Emphasis) -> Self {
        DeletedChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for DeletedChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        DeletedChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for DeletedChild {
    fn from(child: FieldSet) -> Self {
        DeletedChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for DeletedChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        DeletedChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for DeletedChild {
    fn from(child: FieldSetLegend) -> Self {
        DeletedChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for DeletedChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        DeletedChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for DeletedChild {
    fn from(child: Figure) -> Self {
        DeletedChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for DeletedChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        DeletedChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for DeletedChild {
    fn from(child: FigureCaption) -> Self {
        DeletedChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for DeletedChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        DeletedChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for DeletedChild {
    fn from(child: Footer) -> Self {
        DeletedChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for DeletedChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        DeletedChild::Footer(builder.build())
    }
}
impl From<Form> for DeletedChild {
    fn from(child: Form) -> Self {
        DeletedChild::Form(child)
    }
}
impl From<builders::FormBuilder> for DeletedChild {
    fn from(builder: builders::FormBuilder) -> Self {
        DeletedChild::Form(builder.build())
    }
}
impl From<Head> for DeletedChild {
    fn from(child: Head) -> Self {
        DeletedChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for DeletedChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        DeletedChild::Head(builder.build())
    }
}
impl From<Header> for DeletedChild {
    fn from(child: Header) -> Self {
        DeletedChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for DeletedChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        DeletedChild::Header(builder.build())
    }
}
impl From<Heading1> for DeletedChild {
    fn from(child: Heading1) -> Self {
        DeletedChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for DeletedChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        DeletedChild::Heading1(builder.build())
    }
}
impl From<Heading2> for DeletedChild {
    fn from(child: Heading2) -> Self {
        DeletedChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for DeletedChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        DeletedChild::Heading2(builder.build())
    }
}
impl From<Heading3> for DeletedChild {
    fn from(child: Heading3) -> Self {
        DeletedChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for DeletedChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        DeletedChild::Heading3(builder.build())
    }
}
impl From<Heading4> for DeletedChild {
    fn from(child: Heading4) -> Self {
        DeletedChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for DeletedChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        DeletedChild::Heading4(builder.build())
    }
}
impl From<Heading5> for DeletedChild {
    fn from(child: Heading5) -> Self {
        DeletedChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for DeletedChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        DeletedChild::Heading5(builder.build())
    }
}
impl From<Heading6> for DeletedChild {
    fn from(child: Heading6) -> Self {
        DeletedChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for DeletedChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        DeletedChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for DeletedChild {
    fn from(child: HeadingGroup) -> Self {
        DeletedChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for DeletedChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        DeletedChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for DeletedChild {
    fn from(child: HorizontalRule) -> Self {
        DeletedChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for DeletedChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        DeletedChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for DeletedChild {
    fn from(child: Html) -> Self {
        DeletedChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for DeletedChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        DeletedChild::Html(builder.build())
    }
}
impl From<Image> for DeletedChild {
    fn from(child: Image) -> Self {
        DeletedChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for DeletedChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        DeletedChild::Image(builder.build())
    }
}
impl From<InlineFrame> for DeletedChild {
    fn from(child: InlineFrame) -> Self {
        DeletedChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for DeletedChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        DeletedChild::InlineFrame(builder.build())
    }
}
impl From<Input> for DeletedChild {
    fn from(child: Input) -> Self {
        DeletedChild::Input(child)
    }
}
impl From<builders::InputBuilder> for DeletedChild {
    fn from(builder: builders::InputBuilder) -> Self {
        DeletedChild::Input(builder.build())
    }
}
impl From<Inserted> for DeletedChild {
    fn from(child: Inserted) -> Self {
        DeletedChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for DeletedChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        DeletedChild::Inserted(builder.build())
    }
}
impl From<Italic> for DeletedChild {
    fn from(child: Italic) -> Self {
        DeletedChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for DeletedChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        DeletedChild::Italic(builder.build())
    }
}
impl From<Keyboard> for DeletedChild {
    fn from(child: Keyboard) -> Self {
        DeletedChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for DeletedChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        DeletedChild::Keyboard(builder.build())
    }
}
impl From<Label> for DeletedChild {
    fn from(child: Label) -> Self {
        DeletedChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for DeletedChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        DeletedChild::Label(builder.build())
    }
}
impl From<LineBreak> for DeletedChild {
    fn from(child: LineBreak) -> Self {
        DeletedChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for DeletedChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        DeletedChild::LineBreak(builder.build())
    }
}
impl From<Link> for DeletedChild {
    fn from(child: Link) -> Self {
        DeletedChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for DeletedChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        DeletedChild::Link(builder.build())
    }
}
impl From<ListItem> for DeletedChild {
    fn from(child: ListItem) -> Self {
        DeletedChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for DeletedChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        DeletedChild::ListItem(builder.build())
    }
}
impl From<Main> for DeletedChild {
    fn from(child: Main) -> Self {
        DeletedChild::Main(child)
    }
}
impl From<builders::MainBuilder> for DeletedChild {
    fn from(builder: builders::MainBuilder) -> Self {
        DeletedChild::Main(builder.build())
    }
}
impl From<Map> for DeletedChild {
    fn from(child: Map) -> Self {
        DeletedChild::Map(child)
    }
}
impl From<builders::MapBuilder> for DeletedChild {
    fn from(builder: builders::MapBuilder) -> Self {
        DeletedChild::Map(builder.build())
    }
}
impl From<MapArea> for DeletedChild {
    fn from(child: MapArea) -> Self {
        DeletedChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for DeletedChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        DeletedChild::MapArea(builder.build())
    }
}
impl From<Mark> for DeletedChild {
    fn from(child: Mark) -> Self {
        DeletedChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for DeletedChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        DeletedChild::Mark(builder.build())
    }
}
impl From<Menu> for DeletedChild {
    fn from(child: Menu) -> Self {
        DeletedChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for DeletedChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        DeletedChild::Menu(builder.build())
    }
}
impl From<Metadata> for DeletedChild {
    fn from(child: Metadata) -> Self {
        DeletedChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for DeletedChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        DeletedChild::Metadata(builder.build())
    }
}
impl From<Meter> for DeletedChild {
    fn from(child: Meter) -> Self {
        DeletedChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for DeletedChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        DeletedChild::Meter(builder.build())
    }
}
impl From<Navigation> for DeletedChild {
    fn from(child: Navigation) -> Self {
        DeletedChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for DeletedChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        DeletedChild::Navigation(builder.build())
    }
}
impl From<NoScript> for DeletedChild {
    fn from(child: NoScript) -> Self {
        DeletedChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for DeletedChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        DeletedChild::NoScript(builder.build())
    }
}
impl From<Object> for DeletedChild {
    fn from(child: Object) -> Self {
        DeletedChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for DeletedChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        DeletedChild::Object(builder.build())
    }
}
impl From<Option> for DeletedChild {
    fn from(child: Option) -> Self {
        DeletedChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for DeletedChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        DeletedChild::Option(builder.build())
    }
}
impl From<OptionGroup> for DeletedChild {
    fn from(child: OptionGroup) -> Self {
        DeletedChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for DeletedChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        DeletedChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for DeletedChild {
    fn from(child: OrderedList) -> Self {
        DeletedChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for DeletedChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        DeletedChild::OrderedList(builder.build())
    }
}
impl From<Output> for DeletedChild {
    fn from(child: Output) -> Self {
        DeletedChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for DeletedChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        DeletedChild::Output(builder.build())
    }
}
impl From<Paragraph> for DeletedChild {
    fn from(child: Paragraph) -> Self {
        DeletedChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for DeletedChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        DeletedChild::Paragraph(builder.build())
    }
}
impl From<Picture> for DeletedChild {
    fn from(child: Picture) -> Self {
        DeletedChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for DeletedChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        DeletedChild::Picture(builder.build())
    }
}
impl From<Preformatted> for DeletedChild {
    fn from(child: Preformatted) -> Self {
        DeletedChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for DeletedChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        DeletedChild::Preformatted(builder.build())
    }
}
impl From<Progress> for DeletedChild {
    fn from(child: Progress) -> Self {
        DeletedChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for DeletedChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        DeletedChild::Progress(builder.build())
    }
}
impl From<Quote> for DeletedChild {
    fn from(child: Quote) -> Self {
        DeletedChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for DeletedChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        DeletedChild::Quote(builder.build())
    }
}
impl From<Ruby> for DeletedChild {
    fn from(child: Ruby) -> Self {
        DeletedChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for DeletedChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        DeletedChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for DeletedChild {
    fn from(child: RubyParenthesis) -> Self {
        DeletedChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for DeletedChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        DeletedChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for DeletedChild {
    fn from(child: RubyText) -> Self {
        DeletedChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for DeletedChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        DeletedChild::RubyText(builder.build())
    }
}
impl From<Sample> for DeletedChild {
    fn from(child: Sample) -> Self {
        DeletedChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for DeletedChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        DeletedChild::Sample(builder.build())
    }
}
impl From<Script> for DeletedChild {
    fn from(child: Script) -> Self {
        DeletedChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DeletedChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DeletedChild::Script(builder.build())
    }
}
impl From<Search> for DeletedChild {
    fn from(child: Search) -> Self {
        DeletedChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for DeletedChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        DeletedChild::Search(builder.build())
    }
}
impl From<Section> for DeletedChild {
    fn from(child: Section) -> Self {
        DeletedChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for DeletedChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        DeletedChild::Section(builder.build())
    }
}
impl From<Select> for DeletedChild {
    fn from(child: Select) -> Self {
        DeletedChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for DeletedChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        DeletedChild::Select(builder.build())
    }
}
impl From<Slot> for DeletedChild {
    fn from(child: Slot) -> Self {
        DeletedChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for DeletedChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        DeletedChild::Slot(builder.build())
    }
}
impl From<Small> for DeletedChild {
    fn from(child: Small) -> Self {
        DeletedChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for DeletedChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        DeletedChild::Small(builder.build())
    }
}
impl From<Source> for DeletedChild {
    fn from(child: Source) -> Self {
        DeletedChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for DeletedChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        DeletedChild::Source(builder.build())
    }
}
impl From<Span> for DeletedChild {
    fn from(child: Span) -> Self {
        DeletedChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for DeletedChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        DeletedChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for DeletedChild {
    fn from(child: StrikeThrough) -> Self {
        DeletedChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for DeletedChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        DeletedChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for DeletedChild {
    fn from(child: Strong) -> Self {
        DeletedChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for DeletedChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        DeletedChild::Strong(builder.build())
    }
}
impl From<Style> for DeletedChild {
    fn from(child: Style) -> Self {
        DeletedChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for DeletedChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        DeletedChild::Style(builder.build())
    }
}
impl From<SubScript> for DeletedChild {
    fn from(child: SubScript) -> Self {
        DeletedChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for DeletedChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        DeletedChild::SubScript(builder.build())
    }
}
impl From<Summary> for DeletedChild {
    fn from(child: Summary) -> Self {
        DeletedChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for DeletedChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        DeletedChild::Summary(builder.build())
    }
}
impl From<SupScript> for DeletedChild {
    fn from(child: SupScript) -> Self {
        DeletedChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for DeletedChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        DeletedChild::SupScript(builder.build())
    }
}
impl From<Table> for DeletedChild {
    fn from(child: Table) -> Self {
        DeletedChild::Table(child)
    }
}
impl From<builders::TableBuilder> for DeletedChild {
    fn from(builder: builders::TableBuilder) -> Self {
        DeletedChild::Table(builder.build())
    }
}
impl From<TableBody> for DeletedChild {
    fn from(child: TableBody) -> Self {
        DeletedChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for DeletedChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        DeletedChild::TableBody(builder.build())
    }
}
impl From<TableCell> for DeletedChild {
    fn from(child: TableCell) -> Self {
        DeletedChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for DeletedChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        DeletedChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for DeletedChild {
    fn from(child: TableColumn) -> Self {
        DeletedChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for DeletedChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        DeletedChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for DeletedChild {
    fn from(child: TableColumnGroup) -> Self {
        DeletedChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for DeletedChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        DeletedChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for DeletedChild {
    fn from(child: TableFoot) -> Self {
        DeletedChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for DeletedChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        DeletedChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for DeletedChild {
    fn from(child: TableHead) -> Self {
        DeletedChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for DeletedChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        DeletedChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for DeletedChild {
    fn from(child: TableHeader) -> Self {
        DeletedChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for DeletedChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        DeletedChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for DeletedChild {
    fn from(child: TableRow) -> Self {
        DeletedChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for DeletedChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        DeletedChild::TableRow(builder.build())
    }
}
impl From<Template> for DeletedChild {
    fn from(child: Template) -> Self {
        DeletedChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DeletedChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DeletedChild::Template(builder.build())
    }
}
impl From<TextArea> for DeletedChild {
    fn from(child: TextArea) -> Self {
        DeletedChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for DeletedChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        DeletedChild::TextArea(builder.build())
    }
}
impl From<Time> for DeletedChild {
    fn from(child: Time) -> Self {
        DeletedChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for DeletedChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        DeletedChild::Time(builder.build())
    }
}
impl From<Title> for DeletedChild {
    fn from(child: Title) -> Self {
        DeletedChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for DeletedChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        DeletedChild::Title(builder.build())
    }
}
impl From<Track> for DeletedChild {
    fn from(child: Track) -> Self {
        DeletedChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for DeletedChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        DeletedChild::Track(builder.build())
    }
}
impl From<Underline> for DeletedChild {
    fn from(child: Underline) -> Self {
        DeletedChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for DeletedChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        DeletedChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for DeletedChild {
    fn from(child: UnorderedList) -> Self {
        DeletedChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for DeletedChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        DeletedChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for DeletedChild {
    fn from(child: Variable) -> Self {
        DeletedChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for DeletedChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        DeletedChild::Variable(builder.build())
    }
}
impl From<Video> for DeletedChild {
    fn from(child: Video) -> Self {
        DeletedChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for DeletedChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        DeletedChild::Video(builder.build())
    }
}
impl From<WordBreak> for DeletedChild {
    fn from(child: WordBreak) -> Self {
        DeletedChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for DeletedChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        DeletedChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for DeletedChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeletedChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Address(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Article(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Base(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Body(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Button(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Code(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Data(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Details(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Division(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Form(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Head(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Header(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Html(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Image(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Input(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Label(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Link(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Main(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Map(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Object(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Option(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Output(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Search(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Section(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Select(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Small(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Source(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Span(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Style(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Table(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Template(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Time(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Title(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Track(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::Video(child) => std::fmt::Debug::fmt(child, f),
            DeletedChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for DeletedChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeletedChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Address(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Article(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Aside(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Audio(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Base(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DeletedChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Body(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Bold(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Button(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Caption(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Cite(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Code(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Custom(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Data(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::DataList(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Definition(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Details(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Division(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Embed(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Figure(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Footer(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Form(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Head(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Header(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Html(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Image(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Input(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Italic(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Label(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Link(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Main(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Map(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Mark(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Menu(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Meter(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Object(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Option(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Output(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Picture(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Progress(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Quote(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Sample(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Script(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Search(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Section(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Select(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Slot(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Small(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Source(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Span(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Strong(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Style(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Summary(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Table(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Template(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Time(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Title(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Track(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Underline(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Variable(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::Video(child) => std::fmt::Display::fmt(child, f),
            DeletedChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
