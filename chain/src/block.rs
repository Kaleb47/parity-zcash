use hex::FromHex;
use hash::H256;
use ser::{deserialize};
use merkle_root::merkle_root;
use {BlockHeader, Transaction};
use super::RepresentH256;

#[derive(Debug, PartialEq, Clone, Serializable, Deserializable)]
pub struct Block {
	pub block_header: BlockHeader,
	pub transactions: Vec<Transaction>,
}

impl From<&'static str> for Block {
	fn from(s: &'static str) -> Self {
		deserialize(&s.from_hex::<Vec<u8>>().unwrap() as &[u8]).unwrap()
	}
}

impl RepresentH256 for Block {
	fn h256(&self) -> H256 { self.hash() }
}

impl Block {
	pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
		Block { block_header: header, transactions: transactions }
	}

	/// Returns block's merkle root.
	pub fn merkle_root(&self) -> H256 {
		let hashes = self.transactions.iter().map(Transaction::hash).collect::<Vec<H256>>();
		merkle_root(&hashes)
	}

	pub fn transactions(&self) -> &[Transaction] {
		&self.transactions
	}

	pub fn header(&self) -> &BlockHeader {
		&self.block_header
	}

	pub fn hash(&self) -> H256 {
		self.block_header.hash()
	}
}

#[cfg(test)]
mod tests {
	use hex::FromHex;
	use hash::H256;
	use ser::{serialize, deserialize};
	use super::Block;

	#[test]
	fn test_block_parse() {
		let blocks = vec![
			(
				// https://zcash.blockexplorer.com/block/00040fe8ec8471911baa1db1266ea15dd06b4a8a5c453883c000b031973dce08
				// https://zcash.blockexplorer.com/api/rawblock/00040fe8ec8471911baa1db1266ea15dd06b4a8a5c453883c000b031973dce08
				"040000000000000000000000000000000000000000000000000000000000000000000000db4d7a85b768123f1dff1d4c4cece70083b2d27e117b4ac2e31d087988a5eac4000000000000000000000000000000000000000000000000000000000000000090041358ffff071f5712000000000000000000000000000000000000000000000000000000000000fd4005000a889f00854b8665cd555f4656f68179d31ccadc1b1f7fb0952726313b16941da348284d67add4686121d4e3d930160c1348d8191c25f12b267a6a9c131b5031cbf8af1f79c9d513076a216ec87ed045fa966e01214ed83ca02dc1797270a454720d3206ac7d931a0a680c5c5e099057592570ca9bdf6058343958b31901fce1a15a4f38fd347750912e14004c73dfe588b903b6c03166582eeaf30529b14072a7b3079e3a684601b9b3024054201f7440b0ee9eb1a7120ff43f713735494aa27b1f8bab60d7f398bca14f6abb2adbf29b04099121438a7974b078a11635b594e9170f1086140b4173822dd697894483e1c6b4e8b8dcd5cb12ca4903bc61e108871d4d915a9093c18ac9b02b6716ce1013ca2c1174e319c1a570215bc9ab5f7564765f7be20524dc3fdf8aa356fd94d445e05ab165ad8bb4a0db096c097618c81098f91443c719416d39837af6de85015dca0de89462b1d8386758b2cf8a99e00953b308032ae44c35e05eb71842922eb69797f68813b59caf266cb6c213569ae3280505421a7e3a0a37fdf8e2ea354fc5422816655394a9454bac542a9298f176e211020d63dee6852c40de02267e2fc9d5e1ff2ad9309506f02a1a71a0501b16d0d36f70cdfd8de78116c0c506ee0b8ddfdeb561acadf31746b5a9dd32c21930884397fb1682164cb565cc14e089d66635a32618f7eb05fe05082b8a3fae620571660a6b89886eac53dec109d7cbb6930ca698a168f301a950be152da1be2b9e07516995e20baceebecb5579d7cdbc16d09f3a50cb3c7dffe33f26686d4ff3f8946ee6475e98cf7b3cf9062b6966e838f865ff3de5fb064a37a21da7bb8dfd2501a29e184f207caaba364f36f2329a77515dcb710e29ffbf73e2bbd773fab1f9a6b005567affff605c132e4e4dd69f36bd201005458cfbd2c658701eb2a700251cefd886b1e674ae816d3f719bac64be649c172ba27a4fd55947d95d53ba4cbc73de97b8af5ed4840b659370c556e7376457f51e5ebb66018849923db82c1c9a819f173cccdb8f3324b239609a300018d0fb094adf5bd7cbb3834c69e6d0b3798065c525b20f040e965e1a161af78ff7561cd874f5f1b75aa0bc77f720589e1b810f831eac5073e6dd46d00a2793f70f7427f0f798f2f53a67e615e65d356e66fe40609a958a05edb4c175bcc383ea0530e67ddbe479a898943c6e3074c6fcc252d6014de3a3d292b03f0d88d312fe221be7be7e3c59d07fa0f2f4029e364f1f355c5d01fa53770d0cd76d82bf7e60f6903bc1beb772e6fde4a70be51d9c7e03c8d6d8dfb361a234ba47c470fe630820bbd920715621b9fbedb49fcee165ead0875e6c2b1af16f50b5d6140cc981122fcbcf7c5a4e3772b3661b628e08380abc545957e59f634705b1bbde2f0b4e055a5ec5676d859be77e20962b645e051a880fddb0180b4555789e1f9344a436a84dc5579e2553f1e5fb0a599c137be36cabbed0319831fea3fddf94ddc7971e4bcf02cdc93294a9aab3e3b13e3b058235b4f4ec06ba4ceaa49d675b4ba80716f3bc6976b1fbf9c8bf1f3e3a4dc1cd83ef9cf816667fb94f1e923ff63fef072e6a19321e4812f96cb0ffa864da50ad74deb76917a336f31dce03ed5f0303aad5e6a83634f9fcc371096f8288b8f02ddded5ff1bb9d49331e4a84dbe1543164438fde9ad71dab024779dcdde0b6602b5ae0a6265c14b94edd83b37403f4b78fcd2ed555b596402c28ee81d87a909c4e8722b30c71ecdd861b05f61f8b1231795c76adba2fdefa451b283a5d527955b9f3de1b9828e7b2e74123dd47062ddcc09b05e7fa13cb2212a6fdbc65d7e852cec463ec6fd929f5b8483cf3052113b13dac91b69f49d1b7d1aec01c4a68e41ce1570101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff071f0104455a6361736830623963346565663862376363343137656535303031653335303039383462366665613335363833613763616331343161303433633432303634383335643334ffffffff010000000000000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000",
				"00040fe8ec8471911baa1db1266ea15dd06b4a8a5c453883c000b031973dce08",
				"c4eaa58879081de3c24a7b117ed2b28300e7ec4c4c1dff1d3f1268b7857a4ddb"
			),
			(
				// https://zcash.blockexplorer.com/block/000000e869e3a0fa79858a51b4b1d09a6480dcdb37bae63653fcb11a718abf3f
				// https://zcash.blockexplorer.com/api/rawblock/000000e869e3a0fa79858a51b4b1d09a6480dcdb37bae63653fcb11a718abf3f
				"0400000072d43a7895d2774d081ccee922a81d77a8879f46a8aa86d65aebd38e9800000012f742113a92b8f55a2474017e30d09ba7f0b3d9e144db761a77e08d0ec476640000000000000000000000000000000000000000000000000000000000000000868c1358a376011e9e02000000000000030000000000000000000000000000000000000000000000fd4005002b983cb99149af0d05b36f45cc2f4d47af59f79d0253cc3ce8395065da1862fc31a5268d5663f0eccd02dee3b544c93fc45900e65d13466f12c039bbff5a076ed18698035914244c108f8355494df71bbb3c7d084293d3838913485b45a1553f42fc7208ae7d7350299c82639bec1a8fd4e8286a73e3750732e0fb05ad08fd80ceca44dd4d53c581356d4e572cfabff96a4c19248c3a87eab33becee256903ded2916cb3accd75042caf1981510fc1c83476b17e66d9be8f04b9dd8d0d75a9d95bcf3367e023b45f8eb06ce9c3f1d7a16013f4b70bfdcaf3530e3177a8c1f5f901fab59a5a771c31e57c8ece507d729a33383358fc7126093bb7d011d7f392235431292fadf2ffd8a929aa95f957503279afa40ef229290996c6ba136a752b5eef96178754461b9d22a51703bf99dbf710f9ec61e1ccb61828cb52ab04564cb434fbd09e79308d4b4ab27bd5b4838b02a0d77023c54e0cf43e44dfadd75392817c75177219c70c7635cbdcf111117780dec878b785a35e1329118ade96dd4dea211784c16d72dbe14f62935fc66c2ebafffc59cf2488e28ada5e586a9f5752d57e5d3e07ab7b332f90a585ffe72164abece07db6cdf1fc961335f3360d1e3c81d490126bcde56a5e16eb56e6f0126bf44358574843b586984a29f6f2163bb7f217be271dbae0142c7f697768f8d2cb54195e5ef675dab203998961afec029dccaa21d16069267630ef7c00881fa7c2bb761b9c2cdfa8a29d40c1db875b8d7d55f008ef8081a8cfbde7632330ebe0281a0092b5507d6c275269400453a46cb2ae27ccf1e18f3622daf9af3f048803cd27cdd72d287af371e921cbf708eab933a0151725a5a3f1e331b378c348594f0d319faf51dd200ed463245f175d5be68aa1a9804812d94a7334f9bc31bb09ce11d1341712e408069dc6c39789c3de18eb00f8af8e010aa8af5407d3a16ca2c59d59abf522391453992e2e18ad57f55f716d80d9c0e9710e4bd79014be01c0792199274b058386a8ebe9067de29cac0221859174079b8b2311c75320a8c96023b9c01e992601b75cfe8090a65356bcf5ec7ac4c5a69f50794f310b2408cdf10f9d9ec85183f4de6c1ba1da0bd99c9a3180eaabb4b50d11b93775d03fce9289818d4d7bf236c25675a49ba3dae9ca9b0561eebd4785cd1c829f01c2a79bdb8a96c0d2a7719dc9a6c8ec9ec6d70cbf56ab1623a5dab59cf74ac9874f7275b705747846881bcf86c8c824ca3944fa31c8e7b88cad2c5309ae0d4b8796a5f8570916fb3c858cdb60f697ae855e532210759d6aa7050fd4df4c56852dccc1fad5d91cd1ce2aa1719319a4951f31280aa33ddfe4fac2fd9b554e125f02ada0cb6906f85666cb07cd0036661859d428166d17c7c7e1c011c5c0366a61ded942898b37583001f0b2cbe66285a730a4d2e4ed7eaa53a85d3e2f4a0493f95927194d1bc80a38710aeb620e7bebfce81b4c14c744d8f7c131e645861c6abe1f72ba50798b655cd94fa7129a9e0fc0cb58f439d0b65ec9eedee0f00366cd8145503c9f1936b084f435f494488a45d19530f574b6de8fe6d32e2b43948e6ae8561a185772aa18d586163554019383abb2ea02482e82ffd59b4ee01aa7c414a09e9071e2a501ad56c9a410f352d1713103fd94762e023aad97a75086a6c49f2d923e8f15640e70117a01969320fb1992d4bbb9c1bef33c7a22d013ee35a896e46d038179a21ad31bc8a5097150d4da2bfc8f1c16e771d1b5bd7419d4c839a76faf5edeba151603821c52284f7722b3c8125e5c3d3901768fb65bf6d51f3fad0277a6eee79b60d8c7f38f995f2b5a1ed3d677fef0e417deacf8d2707f1c265fedf9750938625fc25628cef592aad4dd3ce54ae1f147de71370201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff06028c0102f401ffffffff02c01f2e01000000002321033bf6b16c6987b017991932dc66dc96ccdbde81e4c0b2ea086d246349bedde903acf0874b000000000017a9147d46a730d31f97b1930d3368a967c309bd4d136a870000000002000000010a141a3f21ed57fa8449ceac0b11909f1b5560f06b772753ca008d49675d45310000000048473044022041aaea8391c0182bf71bd974662e99534d99849b167062f7e8372c4f1a16c2d50220291b2ca6ae7616cd1f1bfddcda5ef2f53d78c2e153d3a8db571885f9adb5f05401ffffffff0000000000011070d900000000000000000000000000d7c612c817793191a1e68652121876d6b3bde40f4fa52bc314145ce6e5cdd2597ae7c48e86173b231e84fbdcb4d8f569f28f71ebf0f9b5867f9d4c12e031a2acc0108235936d2fa2d2c968654fbea2a89fde8522ec7c227d2ff3c10bff9c1197d8a290cca91f23792df8e56aed6c142eaa322e66360b5c49132b940689fb2bc5e77f7877bba6d2c4425d9861515cbe8a5c87dfd7cf159e9d4ac9ff63c096fbcd91d2a459877b1ed40748e2f020cdc678cf576a62c63138d820aba3df4074014bb1624b703774e138c706ba394698fd33c58424bb1a8d22be0d7bc8fe58d369e89836fe673c246d8d0cb1d7e1cc94acfa5b8d76010db8d53a36a3f0e33f0ccbc0f861b5e3d0a92e1c05c6bca775ba7389f6444f0e6cbd34141953220718594664022cbbb59465c880f50d42d0d49d6422197b5f823c2b3ffdb341869b98ed2eb2fd031b271702bda61ff885788363a7cf980a134c09a24c9911dc94cbe970bd613b700b0891fe8b8b05d9d2e7e51df9d6959bdf0a3f2310164afb197a229486a0e8e3808d76c75662b568839ebac7fbf740db9d576523282e6cdd1adf8b0f9c183ae95b0301fa1146d35af869cc47c51cfd827b7efceeca3c55884f54a68e38ee7682b5d102131b9b1198ed371e7e3da9f5a8b9ad394ab5a29f67a1d9b6ca1b8449862c69a5022e5d671e6989d33c182e0a6bbbe4a9da491dbd93ca3c01490c8f74a780479c7c031fb473670cacde779713dcd8cbdad802b8d418e007335919837becf46a3b1d0e02120af9d926bed2b28ed8a2b8307b3da2a171b3ee1bc1e6196773b570407df6b43b51b52c43f834ee0854577cd3a57f8fc23b02a3845cc1f0f42410f363d862e436bf06dbc5f94eddd3b83cdf47cf0acbd7750dff5cba86ea6f1f46a5013e0dc76715d7230e44a038a527cb9033f3eeaeac661264dc6a384788a7cd8aed59589bca6205fe1bd683fa392e7a3c6cc364bba36ad75ee9babf90f7b94071953df95effc0b1c3f542913ed1eb68e15534f9ceb7777c946edf55f129df128c3f767d8d60c4aa0c5e61d00f8e495e78334e2a9feddd9302e9880cb6174d201c89a1d6bc6e83a80cbf80ab3959dcc6cdd12e3d2f6f14d226e6948954f05544941d16ed1d498532722fa39bb985c3224915dd42d70be61217fdcb4aa023251af38b5576ff9eb865a471f2cb2dbc674e401d18014e6119464768778ddcd00907f20279bdecda3880fbbb4d00bb6c5aa3e06113a2f12fcc298f34ccb6bc2c2887b0b064f3bc2e2b507d31e022e65800dd7d30f25266914646bfc07c1eafbbf1e1163c439774b47e8e844799bc8fd06db050f97f5c74ca833e81bcdcf9d864be5746f965ef41838a3535666df867ef79e07068dc7ef809fb0e08e1629bab3215fe36d0f0e0f8c6bb319f93a0f408ff4abbd88c21afaec2e7720674eaceb27efb9144f619bad6f033cbefcebfbe66cabe8286f2ff97b91f4aeef5cbd99a9b862cb904dc085d96238caaad259280ff35caa211e00324f51ff03b6a1cd159cd501faef780ef7f25a98cdcd05ef67596d58d4aea1f9f3e95aae44fd4d4ea679c5e393d4670fb35bf12d036ea731bdfad297303239251a91f9a900e06987eb8e9f5bb1fb847f5ae47e6724ddeb5a3ac01b706a02e494c5547ce338302b4906cf2c91d59a87324322763a12e13a512ace3afb897510ad9ec95aa14ca568a9962da64e5bc7fd15b3e103ab461ee7db3fc9da0a523fc403c11254cd567ca48c8dac5e5b54953e5c754e31def90fff6c56d589a5c4b9a710ccb43cd24988b2fb9336b5508aa553cfdbd1f32dfb4ff16eae066b5fb244bc9058a91898c4ae893eaf0006dae1185c7f553e6e09d12a0a2a9c181c5e4d87c8895b74b0e23a8dc87faf5d6acd5e98cb1df5585f026ae94b77db0e95c5fe22692bd2e70e8e87d07d92b98cdfcc5367e52014163a6e4511d482816259215ee7df246e493523ee51617c318e1a9825f82e73e640fbc2d25c12ce5a07875d489db6a111afdc87061047077030d32de45cd4e575c02a60c4048560bd02cf9203426f589f429b413390ace832b3ddd3dd371750d94f9c34f60a0f1b621b445525d2190a185feaab9e56a079c46236161559713d585a07e94f2316a92fffa7838f1aea39d7846638d16f9b4d1a7dc053e0ddc6620f30e3e798eba900fd25c10c5d6672c9ed7d4d2fa80c0f0137ff24933c37fcd91b19bc7cdd828f7f3f1df0e45cafca795d847e83bca8baa321006581b024306e24c4c2294c0f41b932c1e9f7602f377e8484c7eeb184fab1f747b1dff5b6e2e89f1e5c4232b5a0a41ed6a3775f8942217078b7e035747891cabd2099bfcbf6a8d4680f51265d9e7d05794514f02470e0eb003ad1222cd4fe8bcd077310c5aff274b19608c31f77453d01c9aa9c21a8d9b71de44386aee2145648f7ead471cabed297b8610bba370baa42603f21f5f4640e5bc1a0402d40394e176a0db8cedb33a9d84c48b58d3851617046511946a3700aabe8f69cdb0469ee67776480be090cad2c7adc0bf59551ef6f1ac3119e5c29ab3b82dd945dab00dc4a91d3826c4e488047a4f3ab2d57c0abe1ee7aba304784e7ad211c32c4058fca7b1db2e282132e5ccafe79fc51ab37334f03715f4ad8735b6e03f01",
				"000000e869e3a0fa79858a51b4b1d09a6480dcdb37bae63653fcb11a718abf3f",
				"6476c40e8de0771a76db44e1d9b3f0a79bd0307e0174245af5b8923a1142f712",
			),
			(
				// https://zcash.blockexplorer.com/block/00000000007ef95f986ed8309d0ed6a1b6174c90b9c7f4d0dfc40f7147315e79
				// https://zcash.blockexplorer.com/api/rawblock/00000000007ef95f986ed8309d0ed6a1b6174c90b9c7f4d0dfc40f7147315e79
				"04000000bd3c4ad2cecf0fc1b26e72d31e9ccb4a61083c23ff78757579d35600000000006be99b7c4e1c7d02f8200fa106571b0e9e1fd0ff81a5e053d07c1f5f68bf56237d71967f4b31fbaaf221a958bbe3e6bc02cc3c3fbe2fed80657bd9fb14f3be12b72f1b5ce42f021c000000008c132a8040000924000000000000a500000000000000000000000000fd400500392d6c2bd1ee9fa99041f5f659645122bb2aa73a2a452959a796e296ced7550ec4dfee2a13995b2a180f28cd4b0f86c947138db3da1d4f530146e4beaa6a40fadd2b2f1e002f770f8422fe4c6a7a346959825c02f0510b1dd6209bbcc874c1b74d4ce6d7bb7fc6c4135d4406c6cdf8ea882fd493517baa4790e65cf4551f4547824b11e569d4c79376ca5d0c97128a399b9d4c8ff58df3d9e0c9f01449a9305047f6decc5cc2fb01838b3e74d71e9fc8e25033c34e3fea9116bb649f01dd076e925946cb17db403350714341b959ded59c1691a7df4419f209c902925934d05eadfbe4fd332e1a218988888b3353dfd498c320eda3fed9e0d83ad908b0379a1c5a35571f9b244e1c7be22e0c95d8f3ba2155357086e39f7d25ba649e8dcee3b19e495110ba27c50767a517c3a5b815a51ac5f744cdc020b6b7bd3183c65561dc765f4b57155c31ca89523f0196b3e702513de4fc8363dfc66981a5dc421e20a392f5487a0305cbcec10996dee18e572dc6412cf1ddf15d3f9306a00a54c416ae40e40735f01d77b65e99d398764d241e6bbfa073c1c1a4df648f72deb60bcddd3f9e740f9144935a4c00c9a08497588eee01166501f801b73d02baa90b11bda8ed6d970e453ea166eed51973ac377e939e4fdd5818f4b4087f135ee31abc1b57b89c3f0105c3cfa923bba62e361719415bb384c3fe214c0a2022d86fcd33bf3f41641484bfd71e018735ef8f535293683e99b61fb005f9dab277796a79695524491e0d5991a198fe4b2fe7e7ef3a5a0b6ea5277cb60e2dde8b65e8a40953f890e797aebd753f297bfa1a5912dc60e79091791734c05169cf649361dd94bf1f8355792540d8eab27d9aa70664d144644688e3df108720db673b33f38a4da641c66f0f673282690b7e2b84352f6c1ba491747cc4a4057838d1e2fa468138ba240139e038ebcc4669fedaa55938671551fd3b19479241d0737b38e3a955c243a625b1c8350265045df84405a760d0990af733301b04a7d96b7f55d7ffdea3e326caa4d057e4ee69841668e19d7813fe45309e90ea04d6a43caf926da543f8824230cf52f6d58f18d580063d013ea3ea5cfb6bca36014ed5a02a5adbdaf0e61c3295c7800ba2cf93c46355eaba2b21281c7656b92cd6f239b9ee4cdd72bda74038f8a2f30f81f9a7c7053a8b9c5e0efe690b5df69362fa61da37ab37c5f715c512ae44cc769af434124484af4b7165f3daeae5084e413aa94cc047511d310b5df82edde9e92fca5321d6d4e2d892aa429ab4f7cddef4e1ca906ffd4bfe0723d9334d430a09499752be299d8312787afc345c2032b6b9bbe24d2735a5f4d58a317f69ff31dc5121149b029973c6bc486e9e530c9c3c94e5810c9b2de642c38e4e78b040efeb0ab871557bbdca3e9a3d7fc5021416ed8ba514076260d3d1217efd7170a18eda291a59a7ec3d5c46295d4165d52866d19a1ed3bd39940f6c7a6e3f594b5cd124b22df95febe518c32b3f711f805f2ca7d3316325eaf2b7d628c7cd57549df3a8050d1f22df901ac1c004411fb4d60656224ad3cd8939956a62f9a5c953d14834b22c4ae68d35d81466b307b81206f6970189097428b1307602e251f55cfee43ac5ad87e812f756e848e41327490e2b2453bb7ce2054e47bd25d90c89f59672f6267895114eab31f0b50c4a7226bf0f7166c7d4b5e633ba01d29b507f84360bc51c7008a70c07cb45e26cc7a72da1b38ed69829393952942f1040199cdb6790294526ea6621b5637e05920c84346074c5b609424c24df9a10eda7ff59a7351af3ce2b0f0c01c64df46a7fe1a18ec81576aa740a6b10c5019bfebb4958d110fdc00df73554ddcdfe0e868c6e37a3b0b3e7a222ed6ce6b35e887b15a94a080400008085202f89010000000000000000000000000000000000000000000000000000000000000000ffffffff2a0360da060004b72f1b5ce475a064e5e716a5dc6c5714316d151882eac199a88b50caae6aaa5299bf29a0ffffffff025d5f9b3b000000001976a914130f06e8110c04122ebb08bb49e3453d206cf8e488ac80b2e60e0000000017a914e38130c1e0add9fb67365f61c6978f2c1891a5d387000000000000000000000000000000000000000400008085202f89014adba0e9f805f333511f1e97ec50e95f5b70c78fce34c593f784c96761663d80020000006a47304402202a0ef385827211421f7ad629ba7ddbfbca97682f098d8fde72f0d5764f273d97022017298fc7db27efd1499e55a28c33368f45b29919b9e2fa9ac50e03f5f64151d80121027b1c69c615ade0ac84a934333b36d1c344d77bcc51937868824335c01d25ec29feffffff02b46402b2550000001976a914ed9019d8252f78c7651da69b0adba3014518640b88ac1cd51206000000001976a914df1cfef1073f717a3c7ef433ab03b172565ae91188ac4eda060074da060000000000000000000000000400008085202f89016b46468486b84c3ce8a37a7d1bed68186b692a4a4bdad2314d0b3b8c3d63762dc70000006a4730440220449f8ffaa3346152bea1825a0ff560725540ac616faeda7f334985eb1919122a02207de01f4bf86adc49cb3df34279d883f4192e1f6a296cf240b0a5222a2758a0b60121022539acfb7d76a3d0ecce4278259f3678b4693a87cc6d58e5cd6e66098463c4830000000002600bb101000000001976a9146375971b222b3f18a48f77f92c97bc9338acfc5788ac9b0e0000000000001976a91401a4db5a5b93f1733fdd83746d70b010872a211588ac000000000000000000000000000000000000000400008085202f890128a359daa6090aaa87982c3592b768a9afafb60cd9b9e068aaf0fe2d405300d8000000006b48304502210092b794d3c292abd216265373faaa05c0d017a83acdf9435e6d60d6bcf7ed8bce02206fa6ec0f85cffd6d7c9157c3f4a7e3b6ac5f84664e90f847a4a5d17209119743012102de6af5046400c030ac05e18d187c29a6c40e741706403af97f3a40b719391476feffffff02c6972e44010000001976a914337681dded0736e6c5d26a1385c5ffa6d59beb3488ace06c1f07000000001976a9140c8c7f49860e97b14b62763039e014e825c078c188ac54da060073da060000000000000000000000000400008085202f8902a999deae50bd4b318b1811d443b7c906ef49d919da451bc211c830f7ec432b8e000000006b483045022100d42dfd57dd5ea076bb198cef55e51c2486ebeb037212aa60006b93fe08e8787102206ec26c097084f09e308ad5b9e3a5454a4b9c98ad30a142526879b49044d6b78601210255eb1549442b3e21cbb98213b62ab5340da8b7d173402d7c8cab16c6659fa7f1feffffffb90eeef1500ee2e1a4a46c4bc1a999a4f434db2d6dd7fbe8878b560dcaef03ae000000006a47304402200c11af18aab29245a5ca33e1e02e06d40e3c36bc303d13bcdd106d4489d4ff9f0220718f8741adfc078ba48ccf790de2f64954e218c00ec7c21503d0df41127369870121039d9a916b2b3ae6d0abd556aec9a3a81e54049b9b123aa977e84cd00501039479feffffff020b5b5600000000001976a91444915cdec4626362e51f03045198d9c163152f0388ac4e751200000000001976a914a950d1a0d291b81e3f55a205efe415219080a4a388ac55da060074da060000000000000000000000000400008085202f890001f8680e2e030000001976a914849d7fb393d294aa4cbc3b73591ad3da95e22b1988ac0000000067da060008900e2e0300000001251bb2e003abe1421823eb492fe85ce17a8cf4f6076f34fa5d94772b143057708facfb01cde6f5723e76f8d4c4fab73517f32c2a8f1fb64ceb9f2978af40024c62c69a3890b6156fa8a6378086981bf8ceec2ece0e6cf41b9b34b25352bcd1caa9a5107e0cdce1c56433da4600169bf86f8d088c6243fc172e6626f43373660298157fdc1f0c96dcbd312ebe97b0845c7d4b51d9190b42f4347f7802556064902eacd8264d94b3dcadb14f4c345a6b38b1d80220c6440b708f8f4078f0de3c09f2b2f1a76df4633a07e2d96db6e67ab879729818d9b9d3e84f4505ee0b73daaf02cd848ac7acdce3708544aaf51f07544495ec162f9ab74f3efca1635ed23197568b44f1a266210195369a8959cfba2da3532ec89c50598ab1ff2a839cc2ad2e339e446b224e0530f15514be0f93ea1b7c70fa67787c6a9aac3da402808f8df76daf656d1a351c61c509ea0015c98de8474d7d610e5f8a8e138b135d695e80ef6322dbf389a7af048e6c1c7d821109ca57a86a90c5145feb2eb56d8e15b4ca030000fb748212e9081c616430f441a2a757d8a93fc8b0cd93d6121db78dae700dd148c70c8220bd34dd036d750f818b8bec08dda1408ee7ee35eda8b4f7f0b902e1020400008085202f8900000000000072da060010270000000000000148b1c0668fce604361fbb1b89bbd76f8fee09b51a9dc0fdfcf6c6720cd596083d970234fcc0e9a70fdfed82d32fbb9ca92c9c5c3bad5daad9ac62b5bf4255817ee5bc95a9af453bb9cc7e2c544aa29efa20011a65b624998369c849aa8f0bc83d60e7902a3cfe6eeaeb8d583a491de5982c5ded29e64cd8f8fac594a5bb4f2838e6c30876e36a18d8d935238815c8d9205a4f1f523ff76b51f614bff1064d1c5fa0a27ec0c43c8a6c2714e7234d32e9a8934a3e9c0f74f1fdac2ddf6be3b13bc933b0478cae556a2d387cc23b05e8b0bd53d9e838ad2d2cb31daccefe256087511b044dfae665f0af0fa968edeea4cbb437a8099724159471adf7946eec434cccc1129f4d1e31d7f3f8be524226c65f28897d3604c14efb64bea6a889b2705617432927229dfa382e78c0ace31cc158fbf3ec1597242955e45af1ee5cfaffd789cc80dc53d6b18d42033ec2c327170e2811fe8ec00feadeb1033eb48ab24a6dce2480ad428be57c4619466fc3181ece69b914fed30566ff853250ef19ef7370601f4c24b0125e4059eec61f63ccbe277363172f2bdee384412ea073c5aca06b94e402ba3a43e15bd9c65bbfb194c561c24a031dec43be95c59eb6b568c176b1038d5b7b057dc032488335284adebfb6607e6a995b7fa418f13c8a61b343e5df44faa1050d9d76550748d9efebe01da97ade5937afd5f007ed26e0af03f283611655e91bc6a4857f66a57a1584ff687c4baf725f4a1b32fae53a3e6e8b98bca319bb1badb704c9c1a04f401f33d813d605eef6943c2c52dbc85ab7081d1f8f69d3202aae281bf42336a949a12a7dbbd22abdd6e92996282ebd69033c22cb0539d97f83636d6a8232209a7411e8b03bef180d83e608563ea2d0becff56dc996c2049df054961bfb21b7cbef5049a7dacc18f2c977aa1b2d48291abc19c3c8ea25d2e61901048354b17ce952f6f2248cf3a0eb54c19b507b41d7281c3d227e2b142ff695d8b925a4bb942ed9492a73a17468a8332a367fd16295420bdca6c04d380271f40440709998fce3a3af3e1e505f5402e5dd464dd179cb0eede3d494a95b84d2fb2eb5abb425cf2c712af999c65259c4782a5ec97388324c67738908a5ba43b6db62a10f50cddf9b5039123437c74165921ac8cf4f13292a216baef9d00bd544106b52755986c98a462ade1149f69367e926d88eb92798c0e56cd19a1bcf264fd93293033b758da65c7901eb5b4a17ee265a3312dbc477868da0057e1b3cbf47726dead6ecfcc8e1044c6f311ff0fc83192dc2f75a89626ba33364dac747b63ff3c8337e00332c8783ba9c8dc13cdf0750d7adc3926fbe1279017d50adba35c38c5b810f73abe5d759cd7fb650f6b0a1f78dc1f62fd017090ff4de4cf54c883752ddda68083d4617ed2c38bab8da313965dd3f7b755aec23a2d9e2965d08d2134827a72ffb3bd65b1fd5410da105bfba7a74ddff0928a654aca1ee211ac9dce8019ddcbb52263ce44b2544a314355c1e8c8543f3ed3e883e7a7a8f9e3c7c11f41ab9069854fb21e9b3660a860df19d289d54b29d82522b32d187cde6261eb0a429c3994dff6f37b9ab9102281223e3cd584790a909e05ba0ea1a2d9aef8e571986e98e09312dccaf8e739d718a1edd217dc4c8a5c8a650015405b592a7c674a451d7d1686c7ea6d93e74a8fe4ade12b679ac780457f08a79bfbf96dcf7eefe9a39b99f1ae39d2c5f86aadf156b7d5ce4b2733f307cfe1e1ff6de0ff2006d9cba535b0c40dfb7a98399cdff8e681fc38c7b9aa94ee5eb89432e28d94ee27f238776ba964a87caf58eddbb64771e64de094305a8eb848d2d9ad6373903687d22170f48f1ae8d714514034ee2733857af4747312bb006e6ce3918ede8c730bacc7821b81c1b93bb50b219e79e8e0d74531ed18c1145632d9847d38783b49141ac5353aaa7d125fb2934e681467e16b28090978e74e0b0400008085202f89032d9418dc7d2882fe6be0ed319a1c549f673a622f071bde5eb52f55bdc54f549b2e0100006b483045022100d459951782ab9179738629855d970fe71a0164b3e873f541f4d8437892ebfbc4022007dd68f95d6095ce1c7437af3a0478e70449027410911b5d1cd0e11bd27d7ded012102c72bc522a7d79e6697ec01ff7c35daf3e5ee7c73ceb9b58d5af7d9814f61031dffffffff7e850495c92512a2cfe21f7cf26e1eff4a03ddb26e2d01f252995ba07382911e010000006b483045022100e02e454dad48d9f1700bdead87c33db9a2495275279a252c8348940eab53e82702205d92c7741067d68bad90dd12562a4a192ae33dd6cfd6fe930be7edc867e9c1d0012102c72bc522a7d79e6697ec01ff7c35daf3e5ee7c73ceb9b58d5af7d9814f61031dffffffffebc0256e10a516907f7442877219d20573728055033d6d64661f494e825a2775010000006b483045022100ffe35271c58cfc9584c883e8aeb5b71a51082ba6c775f9c13424b1ee11e5f01e02200e305d36736d1f18743b8d21885b26e0cbc4b08d856820a1baf0251df98010ed012102c72bc522a7d79e6697ec01ff7c35daf3e5ee7c73ceb9b58d5af7d9814f61031dffffffff02e0561d01000000001976a914535b4f24926d5e1df9ae606fcf27ad73e41888c688acde150000000000001976a914587dea6af9830d207c81290d578777b67a34c71f88ac00000000ff64cd1d0000000000000000000000",
				"00000000007ef95f986ed8309d0ed6a1b6174c90b9c7f4d0dfc40f7147315e79",
				"2356bf685f1f7cd053e0a581ffd01f9e0e1b5706a10f20f8027d1c4e7c9be96b",
			)
		];

		for (origin_block, origin_block_hash, origin_merkle_root) in blocks {
			// check that block is parsed and serialized back
			let origin_block = origin_block.from_hex::<Vec<u8>>().unwrap();
			let parsed: Block = deserialize(&origin_block as &[u8]).unwrap();
			let serialized = serialize(&parsed).take();
			assert_eq!(origin_block, serialized);

			// check that block hash is equal to original
			let origin_block_hash = H256::from_reversed_str(origin_block_hash);
			assert_eq!(origin_block_hash, parsed.hash());

			// check that merkle root is equal to original
			let origin_merkle_root = H256::from_reversed_str(origin_merkle_root);
			assert_eq!(origin_merkle_root, parsed.merkle_root());
		}
	}
}
