use web3::types::{H256, Address, FilterBuilder};
use ethabi;

fn web3_topic(topic: ethabi::Topic<ethabi::Hash>) -> Option<Vec<H256>> {
	let t: Vec<ethabi::Hash> = topic.into();
	Some(t.into_iter().map(|x| H256(x)).collect())
}

pub fn web3_filter(filter: ethabi::TopicFilter, address: Address) -> FilterBuilder {
	let t0 = web3_topic(filter.topic0);
	let t1 = web3_topic(filter.topic1);
	let t2 = web3_topic(filter.topic2);
	let t3 = web3_topic(filter.topic3);
	FilterBuilder::default()
		.address(vec![address])
		.topics(t0, t1, t2, t3)
}