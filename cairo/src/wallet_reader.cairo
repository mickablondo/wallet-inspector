use starknet::ContractAddress;

#[starknet::interface]
trait IWalletReader<TContractState> {
    fn add_watched_address(ref self: TContractState, address: ContractAddress);
    fn get_watched_count(self: @TContractState) -> u32;
    fn get_watched_address(self: @TContractState, index: u32) -> ContractAddress;
}

#[starknet::contract]
mod WalletReader {
    use starknet::ContractAddress;
    use starknet::storage::StoragePointerWriteAccess;
    use starknet::storage::StoragePointerReadAccess;
    use starknet::storage::Map;
    use starknet::storage::StorageMapWriteAccess;
    use starknet::storage::StorageMapReadAccess;

    #[storage]
    struct Storage {
        owner: ContractAddress,
        watched_count: u32,
        watched_addresses: Map<u32, ContractAddress>,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.owner.write(owner);
        self.watched_count.write(0);
    }

    #[abi(embed_v0)]
    impl WalletReaderImpl of super::IWalletReader<ContractState> {
        fn add_watched_address(ref self: ContractState, address: ContractAddress) {
            let count = self.watched_count.read();
            self.watched_addresses.write(count, address);
            self.watched_count.write(count + 1);
        }

        fn get_watched_count(self: @ContractState) -> u32 {
            self.watched_count.read()
        }

        fn get_watched_address(self: @ContractState, index: u32) -> ContractAddress {
            self.watched_addresses.read(index)
        }
    }
}