"use client"

import Search from "@/components/search"
import { usePatientStore } from "@/store/patient-management"
import { Button, Flex, Icon, Text } from "@chakra-ui/react"
import { usePathname, useRouter, useSearchParams } from "next/navigation"
import { FaFilter, FaUserPlus } from "react-icons/fa6"
import { IoFilter } from "react-icons/io5"

export const PatientListHeader = () => {
  const search = usePatientStore(state => state.search);
  const searchPatient = usePatientStore(state => state.searchPatient);

  const params = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();

  const onAddPatient = () => {
    const param = new URLSearchParams(params);
    param.delete('id');
    param.set('concent_input', "true")

    const newUrl = `${pathname}?${param.toString()}`;
    router.push(newUrl);
  }

  const onFillSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = e.target;
    searchPatient(value);
  }

  const onClearSearch = () => {
    searchPatient('');
  }

  return (
    <Flex w={'full'} direction={'column'} gap={8}>
      <Text fontSize={{ base: 'lg', xl: 'xl' }} fontWeight={'bold'}>
        Patient Management
      </Text>
      <Flex align={'center'} gap={4}>
        <Search
          defaultValue={search}
          onFillSearch={onFillSearch}
          onClearSearch={onClearSearch}
        />
        <Button
          variant="outline"
          size={'lg'}
          rounded={'lg'}
          borderColor={"neutral.300"}
          p={2}
        >
          <Icon as={FaFilter} boxSize={4} color={'neutral.400'} />
        </Button>
        <Button
          variant="outline"
          size={'lg'}
          rounded={'lg'}
          borderColor={"neutral.300"}
          p={2}
        >
          <Icon as={IoFilter} boxSize={4} color={'neutral.400'} />
        </Button>
        <Button
          colorScheme="primary"
          bg={"primary.700"}
          rounded={"lg"}
          size={'lg'}
          fontSize={'xs'}
          leftIcon={
            <Icon as={FaUserPlus} boxSize={4} />
          }
          onClick={onAddPatient}
        >
          Add Patient
        </Button>
      </Flex>
    </Flex>
  )
}