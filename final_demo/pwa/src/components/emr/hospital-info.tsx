import { useEffect } from "react";
import { useParams } from "next/navigation";
import { Flex, Icon, Skeleton, Text, useDisclosure } from "@chakra-ui/react";
import { FaChevronRight } from "react-icons/fa6";

import { providerCanisterId } from "@/config/canisters/providers.canister";
import { GetProviderBatchRequest } from "@/declarations/provider_registry/provider_registry.did";
import { ProviderActor, useProviderQuery } from "@/services/providers";

import DetailHospitalInfo from "@/components/emr/detail-hospital";

const HospitalInfo = () => {
  const { isOpen, onOpen, onClose } = useDisclosure();
  const params = useParams();

  const { provider_id } = params;

  const {
    data,
    call: getHospitalInfo,
    loading
  } = useProviderQuery({
    functionName: "get_provider_batch",
    onError(error) {
      console.log(error)
    },
  })

  useEffect(() => {
    if (provider_id) {
      const request: GetProviderBatchRequest = {
        ids: [provider_id as string]
      }
      // @ts-expect-error
      getHospitalInfo([request]);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [provider_id])

  if (loading) {
    return <Skeleton height={20} w={'full'} rounded={"xl"} />
  }

  return (
    <Flex
      align={'center'}
      justify={'space-between'}
      gap={5}
      _hover={{ textDecoration: 'underline' }}
      onClick={onOpen}
    >
      <DetailHospitalInfo
        isOpen={isOpen}
        onClose={onClose}
        hospital={data as any}
      />

      <Flex direction={"column"} gap={0}>
        <Text fontSize={'lg'} fontWeight={'bold'} color={'neutral.700'}>
          { // @ts-expect-error
            data?.providers[0].V1.display_name
          }
        </Text>
        <Text fontSize={'sm'} color={'neutral.400'}>
          { // @ts-expect-error
            data?.providers[0].V1.internal_id
          }
        </Text>
      </Flex>
      <Icon as={FaChevronRight} boxSize={5} color={'neutral.400'} />
    </Flex>
  )
}

export default function EMRHospitalInfo() {
  return (
    <ProviderActor
      canisterId={providerCanisterId}
    >
      <HospitalInfo />
    </ProviderActor>
  )
}