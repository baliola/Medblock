import { Flex, Skeleton } from "@chakra-ui/react";

export const EMRReportLoading = () => {
  return (
    <Flex w={"full"}>
      <Skeleton w={"full"} rounded={"xl"} h={"80dvh"} />
    </Flex>
  )
}

export const EMRHistoryLoading = () => {
  return (
    <Flex w={"lg"} direction={'column'} gap={5}>
      {Array.from({ length: 5 }).map((_, index) => (
        <Skeleton key={index} w={"full"} rounded={"xl"} h={"8rem"} />
      ))}
    </Flex>
  )
}

export const EMRProfileLoading = () => {
  return (
    <Flex w={"md"} direction={'column'} gap={5}>
      <Skeleton w={"full"} rounded={"xl"} h={"30dvh"} />
      <Skeleton w={"full"} rounded={"xl"} h={"10dvh"} />
      <Skeleton w={"full"} rounded={"xl"} h={"20dvh"} />
    </Flex>
  )
}