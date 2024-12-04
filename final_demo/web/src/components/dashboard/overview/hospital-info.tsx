"use client"

import { assets } from "@/constants/assets";
import { Provider, ProviderInfoRequest, ProviderInfoResponse } from "@/declarations/provider_registry/provider_registry.did";
import { useProviderQuery } from "@/services/providers";
import { Flex, Image, Skeleton, SkeletonText, Text } from "@chakra-ui/react";
import { Principal } from "@dfinity/principal";
import { useUserPrincipal } from "@ic-reactor/react";
import { useEffect, useState } from "react";

const Hospital = ({ data }: { data: Provider[] | undefined }) => {
  if (!data) return;

  return (
    <Flex align={'center'} gap={5} borderBottom={'1px'} borderColor={"neutral.100"} pb={5}>
      <Image
        src={assets.hospital_default}
        alt={"Hospital Image"}
        w={200}
        h={100}
        objectFit={"cover"}
        borderRadius={10}
        loading="eager"
        fallback={
          <Skeleton w={200} h={100} borderRadius={10} />
        }
      />
      <Flex direction={"column"} color={"neutral.700"} gap={1}>
        <Text fontSize={"2xl"} fontWeight={"bold"}>
          {data[0]?.V1.display_name}
        </Text>
        <Text fontSize={"md"}>
          {data[0]?.V1.internal_id}
        </Text>
      </Flex>
    </Flex>
  )
}

const HospitalLoading = () => {
  return (
    <Flex gap={3} align={'center'}>
      <Skeleton
        w={200}
        h={100}
        rounded={"xl"}
      />
      <Flex direction={"column"} gap={3}>
        <SkeletonText w={72} skeletonHeight={6} noOfLines={1} />
        <SkeletonText w={60} skeletonHeight={6} noOfLines={2} />
      </Flex>
    </Flex>
  )
}

export default function HospitalInfo() {
  const principal = useUserPrincipal();
  const [provider, setProvider] = useState<ProviderInfoResponse>();

  const {
    loading: hospitalLoading,
    call: getProviderInfo
  } = useProviderQuery({
    functionName: "get_provider_info_with_principal",
    refetchOnMount: false
  });

  const checkUserRegistration = async () => {
    if (!principal) return;

    const request: ProviderInfoRequest = {
      provider: Array(principal as unknown as Principal),
    };

    try {
      // @ts-ignore
      const data = await getProviderInfo([request]);
      setProvider(data);
    } catch (err) {
      console.log(err)
    }
  };

  useEffect(() => {
    if (principal) {
      checkUserRegistration();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [principal]);

  return hospitalLoading
    ? <HospitalLoading />
    : <Hospital data={provider?.providers} />
}