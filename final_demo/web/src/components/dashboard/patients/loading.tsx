import { Flex, Skeleton } from "@chakra-ui/react";

export const EMRLoading = () => {
  return (
    <Flex
      w={'xl'}
      bg={'purple.50'}
      transition={'all 0.3s'}
      direction={'column'}
      p={7}
      gap={8}
    >
      <Skeleton height={14} width={"full"} rounded={'xl'} />
      <Skeleton height={100} width={"full"} rounded={'xl'} />
      <Skeleton height={500} width={"full"} rounded={'xl'} />
    </Flex>
  )
}

export const PatientsLoading = () => {
  return (
    <Flex w={'full'} direction={'column'} gap={8}>
      <Skeleton height={24} width={"full"} rounded={'lg'} />

      <Flex w={'full'} justify={'center'} direction={'column'} gap={4}>
        {Array.from({ length: 10 }).map((_, index) => (
          <Skeleton key={index} height={8} width={"full"} rounded={'lg'} />
        ))}
      </Flex>

      <Flex justify={'space-between'} align={'center'} w={'full'} gap={5}>
        <Skeleton height={8} width={"40rem"} rounded={'lg'} />
        <Skeleton height={8} width={"20rem"} rounded={'lg'} />
      </Flex>
    </Flex>
  )
}