use super::RescueParams;
use crate::ff::{PrimeField};
use crate::pairing::bn256::Fr as Fr;

pub struct BN256Rescue{

    mds_matrix: Vec<Vec<Fr>>,
    constants: Vec<Vec<Fr>>,
    padding_constant: Fr,
    rescue_alpha: u64,
    rescue_inalpha: [u64; 4],
}

impl BN256Rescue {
    const NUM_ROUNDS: usize = 22;
    const R: usize = 2;
    const C: usize = 1;
}

impl RescueParams<Fr> for BN256Rescue {

    fn default() -> Self {
        
        let rescue_alpha = 5;
        let rescue_inalpha : [u64; 4] = [14981214993055009997, 6006880321387387405, 10624953561019755799, 2789598613442376532];
        
        let mds_matrix_str = [
            ["14366617689545171512885630617493315423221003212775080528716194410406385797123", 
             "11669903372380417111564101248044843492677474083758127173278100375530997239646", 
             "12518689353812988611826133151519446914438378805919990784203213141973222136491"],

            ["16113693634217276313408119134058368077707827312758157122615723310076309119360",
             "12212707699822849874681481478090890021107428496380159625678863004344035414402",
             "3174515694892552047402832494502838046388438385647544562189366373012592113700"],

            ["11049864798079382664679049994780275401378093953743760155518133156381536348085",    
             "18247406515932203435137147369840265427442199131126094773161878977568464677",
             "1822354681651247527066214946589058520741863922523505999655701777518300692216"]
        ];

        let constants_str = [
            ["125268824893673750740981672487319552118284585928601062179614039156391142076", 
             "1217936983375986419166620289108699826584283044420897749879566240117259673585", 
             "7604215882834144472344120868629315951188423904795034819712161151505858982283"],

            ["18126952981492472572034414532879596156466400176135917923640454622255910535791", 
             "17733012029117542512701501297263932367246220347265258384592998508578279017164", 
             "10233108511259389251634015719132180048329109959272773015957099677269770470229"],

            ["6470041147806175671897841366251780237084545772757406859200289116263251068545", 
             "5399086520168397704598561653820612267126531489661216686736661589777243972719", 
             "6835051715159324636286308924433266985341507346837823346807503954483989601744"],

            ["3802562285559611365922222126534578154201178805388183097251948473250315985318", 
             "6640042607971884352435860788645183198492244896167355979975199642447856841489", 
             "11123366711180168477974970331173763188945632272089587098362612307891159811343"],

            ["8369608177447988858371880226178570942337791066930486746490403957404846519525", 
             "21421338822105891098434020132545313841357557842078313883528891249893789863196", 
             "20163063836443688109731233433975041265069032062160697503335821662923559712232"],

            ["14729826135709551412743863072282364081094356966167292742873354847640814667189", 
             "6503863299835337010065517856415574950011429685053494351855724376672823872373", 
             "17965152226768509168694679972864680441330501790295789928061195893448005906301"],

            ["13154448569183427833189856408276142534840495448410616794216268789957453739847", 
             "7280336819919753185895263302700091223734469022059574929209417569139599899849", 
             "7917129947315683295847281136493652917952275838912970849432148467322562641200"],

            ["21487585296969547282400974134348549933531004691756087271307084576558527558805", 
             "13024463798247233036736313416571449530731802936093026349938717456682243832480", 
             "18073049018104305107104293714468120745502211252870014050773110774352593138825"],
            
            ["2281943901148734211071923597421908017438574458498758399296469663063546920644", 
             "6912482957939447938164131053692360492794250921011998612612545111236078880640", 
             "8173563533172021464243321207419078615342743716242345198246014009502586604464"], 

            ["15449708580995998916280163442625475213081848741570171334243273560544379406508", 
             "21714447646176443908404086343339066098650669361542520892511884849787402383923", 
             "1155229388552644749540744915931688420226108810310447485012071598890689315595"], 
      
            ["2159987821079745988465710913778043938137135172341445493263544517513461743615", 
             "18433046996625867817319039099804430901159261476896632719878416743264749944780", 
             "3223254482119970709626350289623626556114875266107102827126952167052044210452"],

            ["17131477484330150818757545334923419366048679831074661150782134246162899521434", 
            "12143467402896144223337034092823705643975033014734078481554351198858294607599", 
            "12571081425602455280683854442901372178952198548106610062457225966760700056286"],

            ["6524538825783759823690475111929924557599511812857326048645713154067125181379", 
            "10153252800091414913417042983879593218051822876155916407859022593390736297616", 
            "10879993658148028126233574841890905414112833685909393891586751028150723955698"],

            ["3337477051553821546822585200019384185521721256316376591176837565071045747348", 
            "4280079743896645981464241138831272369383008272070910326288480525058107815983", 
            "17805432009427663908059538823116348426828426745399687285536496732025064817305"],

            ["3781369426209952043400107246354316896206896973712922101856175776859502689273", 
            "20343002797559872340055728693088057327723972623935950649727826151687448453493", 
            "7907120604846220431175585481954693393262473234480983330899162084779196090155"],

            ["13748866824587399938772290978257012919462145821706386435394100448426570482059", 
            "12137530792372529125453520128758112250274645720380454826488294092393862280467", 
            "21195032679819951493095880355197939273868125452844588542213833939701659006114"],

            ["759453406267676203738987437740941602024171625354896420238434489023395290735", 
            "841381224749446315607373044524328208108095469123626311625343391204036338242", 
            "4584852225336216184419273116823121600222672016699686310408624265079839608052"],

            ["4936181579294427508658478496145744644299816909659029396083910763296339558694", 
            "15834255304947160080117302161203443276260494198950185838603586233851505119855", 
            "16477456621478881057864471083770104208941170484460306001185632419493622953120"],

            ["13603258209758978780820163099394808038737840144794801035849112743613996779752", 
            "1883705845864996406287906736476844792273360182672739188239788040019645723208", 
            "3725704013502309723964329695508162885014604370602926250340017109939735647022"],

            ["2745817819602894588257693347368086734894299351684897213497809824707155063557", 
            "1173534993565162912196699334872821300034129830766966620174875852153522088703", 
            "4051596295771480047513702674704422803715510938421882195819281641301529265683"],

            ["20058303830719260118226464130444625454129698729075997666597689426378197856452", 
            "2873607654187758511117445763927793942218322385412162038043767816034230323441", 
            "2226422971004738045774707831115762752697002132258921585207279789717217963987"],

            ["3116488732975494528647410296564839086395254377697038420361288779776720404816", 
            "1409942203363505033793904969407191113605866597126091894060693250287411226722", 
            "583281506590763431212779592386066549318664394126984942147876396187995114459"],

            ["12825614406599557714427771444844220658206767344949401097637604132033551705896", 
            "11993200001719586657825273222034223403401017310638964575755671744388458096365", 
            "21537838490862439361119641443426373886326661110506428353207753458454818521673"],

            ["19892986735156739525822071858278333814007742822752690441961972110161521195244", 
            "4230137440214263265721647031335860396918599047041048851329991476576173323602", 
            "16020222753380549300898385769366326167685716477921503480912602642054011304655"],

            ["17790551669536238013520227632001127144192809895060366054163897535971116819611", 
            "7858739248031691616194102362425676143255773530118000624480237495055492908224", 
            "14387391628885808189130959669593706381949750168477954665828427355162839735652"],

            ["20882565375324680052446988058574817376003941899117503385770613590146495791316", 
            "9258887061892833181045389427128279007558456085646194540042760100673496518252", 
            "15855763630649228625854418881831765730944908534008140392765735752908433901135"],

            ["3957178044171406439323705715015648662873912437721319942465847759569981266061", 
            "19630450804321087483298414535602874987436363602993665968896511087417281028480", 
            "15973429579152589851658575191896865929101036262152649100091997677013443186371"],

            ["15902508609617049168004788529294823359225473002955229139723827393783904582443", 
            "9760994085494033903601676945009872653539744871583368335307869229920218140460", 
            "16923328121388483935147927361631681343660942029204967010353109981276463470156"],

            ["4411323803940819659514725040544625234482996757001647664706151457711332057844", 
            "6074827849923495631597562287927954079876104347952596813624507936998171322663", 
            "15762220346766094973070958607405167070835822017687969026919916340862628901895"],

            ["14634056259702650783369750202132776686465788338156764278730792837535216241846", 
            "1696872145492547918462272180301168223544782466333082021168059828568647678113", 
            "38897959508753480408557887319295419939091891768991086840809321951913141289"],

            ["19786670950537189297304141651848610637480244180459756995304456352683876502603", 
            "7069280114986415815479450596793609510107505267094013617128126542387723087446", 
            "20577589382701943783916687237869930671125210715743321951061894156015561300719"],

            ["3440067990636996470321137696297248350366714234863637530969251503281684593363", 
            "13717085734552579707015855591054175596886140888441143103094183854699440516157", 
            "5310614912555096657108161538183239499820591008107242699269815385272637811298"],

            ["17178027477860742359302457357204231977097417192635014415875305221934900435769", 
            "21602542153458205242604551690254012317555615211354703786102615538762140108095", 
            "18957653748303320148001274461194870839225988003760090560085885242892535659594"],

            ["17435503400644059881532107969973695413100535836622662448181694489894045126671", 
            "376285232124603765781472724903993673099474029861045446870478203017093140501", 
            "15162411859728804759451636792870867091573202480485047249063744979936259864466"],

            ["2858310627117214970424277593201334606301572086638708239718742740383767242043", 
            "1216640947259021077523919658480572221832789138152038429340757664790907755979", 
            "7051582545036843497294366485899534121935419188193363349683219545195747858689"],

            ["17996724154602894138631480414276791473497092966586370018481745743728285185355", 
            "10066534526304931324622493625988366659683498695327789068960825914355547335162", 
            "9523276127926810324549611245561981105543567189129997838993994799554153359930"],

            ["4620352022942048598594505086820176995800671738982136874279387735976077958985", 
            "10347267971618378486363591611333494289758787581972440486294285373450479647871", 
            "15486704620537359430389650265915168553413083162928255865833949446124408378925"],

            ["10245058626041259589797394853865076841850532939896636828507316148606771067021", 
            "21782495637085956008989053857750215717165964188642471336683098345265134354117", 
            "19397554514956444696998985277024701644374750834691420578986433482248678043094"],

            ["21746059810196956986654039081652663741157572848779282271004736264314069147943", 
            "127006077752406756354244642831589903310314231383128353402870221013882844717", 
            "14144654567242022548836106388208937049633003514566318929006560072901590918732"],

            ["7217292415481867560069318355282837627912076647923428449787229316718024595905", 
            "1408616106853205894217112159714693378397744898475318517841345470855272669973", 
            "4224731256787236899368391369455629818823804335147266049879799628526445803846"],

            ["18300435291525274463840917442823092445200360163290354886123411281243716170885", 
            "21469909330402099195385178588230898658557909587239075534556897413823405915538", 
            "18936006589415388462767049770469341749960683691311567088339390964870659609856"],

            ["5614074308715622122137151373955524682953805654135728650350829470089653909977", 
            "15478044148457681944544872203870291672405995967460806099397025035009249431301", 
            "13115836706802903818976133550926321871881559176461464724579686492785927256089"],

            ["10018590390182036867294649198196520543007363451415610886877336821406146395557", 
            "1518742958163975367742344171588022057740799743281618241709361612934217016252", 
            "13811220167155276398158066133422231347770058279898691872698071284126026354820"],

            ["1090887593924299664389582805562177248386226012299557451198025008870340060005", 
            "6365901415835753692239845838380733712046564406764820615023030063358761495424", 
            "19286928634191270373952976460943126785231826614058508389924470728786546103591"],

            ["10545510796687736184604488392146586382718124251748935247776239827754507245160", 
            "13127602808502594635928812395265951741494322953610715893016119126236191850405", 
            "1733729887356220071823959084451630845362501616122571610689030538591294656426"],
        ];

        let mut mds_matrix : Vec<Vec<Fr>> = vec![];
        for row in mds_matrix_str.iter() {
            let mut temp : Vec<Fr> = vec![];
            for elem in row.iter() {
                temp.push(Fr::from_str(elem).expect("should convert"));
            }
            mds_matrix.push(temp);
        };

        let mut constants : Vec<Vec<Fr>> = vec![];
        for row in constants_str.iter() {
            let mut temp : Vec<Fr> = vec![];
            for elem in row.iter() {
                temp.push(Fr::from_str(elem).expect("should convert"));
            }
            constants.push(temp);
        };

        // currently padding with answer for the main qestion of the universe
        let padding_constant : Fr = Fr::from_str("42").expect("should convert");

        BN256Rescue{
            mds_matrix,
            constants,
            padding_constant,
            rescue_alpha,
            rescue_inalpha,
        }
    }
       
    fn get_num_rescue_rounds(&self) -> usize {
        Self::NUM_ROUNDS
    }
    
    fn c(&self) -> usize {
        Self::C
    }
    
    fn r(&self) -> usize {
        Self::R
    }
    
    fn t(&self) -> usize {
        Self::C + Self::R
    }
    
    fn get_mds_matrix(&self) -> &Vec<Vec<Fr>> {
        &self.mds_matrix
    }
    
    fn get_constants(&self) -> &Vec<Vec<Fr>> {
        &self.constants
    }
    
    fn padding_constant(&self) -> &Fr {
        &self.padding_constant
    }

    fn rescue_alpha(&self) -> u64 {
        self.rescue_alpha
    }

    fn rescue_invalpha(&self) -> &[u64; 4] {
        &self.rescue_inalpha
    }

}

