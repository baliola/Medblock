"use client"

import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { Button, Flex, Icon, Text, } from "@chakra-ui/react";
import { FaX } from "react-icons/fa6";

import { PatientActor } from "@/services/patients";
import InputConcentCode from "./input";
import { patientCanisterId } from "@/config/canisters/patient.canister";

export default function ConcentCode() {
  const params = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();

  const onClose = () => {
    const param = new URLSearchParams(params);
    param.delete('id');
    param.delete('concent_input');

    const newUrl = `${pathname}?${param.toString()}`;
    router.push(newUrl);
  }

  return (
    <Flex
      w={'24vw'}
      bg={'primary.100'}
      transition={'all 0.3s'}
      direction={'column'}
      py={7} px={4}
      gap={8}
      h={'full'}
      justify={'space-between'}
    >
      <Flex align={'center'} gap={5}>
        <Button size={'xs'} rounded={'full'} colorScheme="red" p={0}>
          <Icon as={FaX} onClick={onClose} />
        </Button>
        <Text
          fontSize={{ base: 'md', xl: 'lg' }}
          fontWeight={'bold'}
        >
          Add Patient
        </Text>
      </Flex>

      <PatientActor
        canisterId={patientCanisterId}
      >
        <InputConcentCode />
      </PatientActor>
    </Flex>
  )
}