import { Flex, Icon, Skeleton, Text, useDisclosure } from "@chakra-ui/react";
import { FaHospital } from "react-icons/fa";
import { BiChevronRight } from "react-icons/bi";
import { useSearchParams } from "next/navigation";
import { useProviderQuery } from "@/services/providers";
import { Fragment, useEffect } from "react";
import { GetProviderBatchRequest } from "@/declarations/provider_registry/provider_registry.did";
import DetailHospitalInfo from "./detail-hospital";

export default function HospitalInfo() {
  const { isOpen, onOpen, onClose } = useDisclosure();

  const params = useSearchParams();
  const provider = params.get("provider") || null;

  const { call, data, loading } = useProviderQuery({
    functionName: "get_provider_batch",
    refetchOnMount: false,
  })

  useEffect(() => {
    if (provider) {
      const request: GetProviderBatchRequest = {
        ids: [provider]
      }
      // @ts-expect-error
      call([request]);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [provider]);

  if (!provider) return null;

  if (loading) return <Skeleton height={12} w={"md"} rounded={"xl"} />

  return (
    <Fragment>
      <DetailHospitalInfo
        isOpen={isOpen}
        onClose={onClose}
        hospital={data as any}
      />
      <Flex
        align={'center'}
        py={3}
        gap={32}
        w={"fit-content"}
        rounded={'xl'}
        transition={"all .3s"}
        cursor={"pointer"}
        _hover={{ px: 3, bg: "primary.200" }}
        onClick={onOpen}
      >
        <Flex align={'center'} gap={4}>
          <Icon as={FaHospital} boxSize={4} color="#06B8EE" />
          <Text fontSize={'md'} fontWeight={'bold'} color={"neutral.700"}>
            {
              // @ts-expect-error
              data?.providers[0].V1.display_name
            }
          </Text>
        </Flex>
        <Icon as={BiChevronRight} boxSize={8} color={"neutral.500"} />
      </Flex>
    </Fragment>
  )
}