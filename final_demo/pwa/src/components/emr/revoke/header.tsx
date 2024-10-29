"use client"

import Search from "@/components/input/search";
import { emrBackNavigation } from "@/constants/contents/emr/back-navigation";
import { useConsentStore } from "@/store/consent-store";
import { Flex, Icon, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";
import { FaChevronLeft } from "react-icons/fa6";

const BackNavigation = () => {
  const router = useRouter();
  const { revoke } = emrBackNavigation;

  return (
    <Flex
      align={'center'}
      w={'fit-content'}
      gap={5}
      onClick={() => router.push(revoke.redirect)}
      color={'neutral.500'}
    >
      <Icon as={FaChevronLeft} boxSize={5} />
      <Text
        fontSize={'md'}
      >
        {revoke.label}
      </Text>
    </Flex>
  )
}

export default function EMRRevokeHeader() {
  const search = useConsentStore(state => state.search);
  const searchByHospitalName = useConsentStore(state => state.setSearch);

  const onFillSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = e.target;
    searchByHospitalName(value);
  }

  const onClearSearch = () => {
    searchByHospitalName("");
  }

  return (
    <Flex direction={'column'} gap={5} pos={'sticky'} top={-5} bg={'white'} zIndex={1} py={2}>
      <BackNavigation />
      <Search
        defaultValue={search}
        onFillSearch={onFillSearch}
        onClearSearch={onClearSearch}
      />
    </Flex>
  )
}