import { Button, Flex, Icon, Text } from "@chakra-ui/react";
import HomeBanner from "./banner";
import Search from "../input/search";
import { IoMdRefresh } from "react-icons/io";
import { useEMRStore } from "@/store/emr-store";
import { homeHeader } from "@/constants/contents/home/header";

export default function HomeHeader({ refreshData }: { refreshData: () => void }) {
  const search = useEMRStore(state => state.search);

  const searchByHospitalName = useEMRStore(state => state.setSearchByHospitalName);

  const onFillSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = e.target;
    searchByHospitalName(value);
  }

  const onClearSearch = () => {
    searchByHospitalName("");
  }

  return (
    <Flex direction={'column'} gap={5}>
      <Flex direction={'column'} gap={5} bg={'white'}>
        <HomeBanner />
      </Flex>
      <Flex direction={'column'} gap={3} pos={'sticky'} top={-5} bg={'white'} py={2}>
        <Text fontSize={'xl'} fontWeight={'bold'}>
          {homeHeader.title}
        </Text>
        <Flex justify={'space-between'} align={'center'} gap={5}>
          <Search
            defaultValue={search}
            onFillSearch={onFillSearch}
            onClearSearch={onClearSearch}
          />
          <Button
            size={'lg'}
            rounded={'xl'}
            p={2}
            onClick={refreshData}
          >
            <Icon as={IoMdRefresh} boxSize={5} />
          </Button>
        </Flex>
      </Flex>
    </Flex>
  )
}