import { Flex, Skeleton } from "@chakra-ui/react"

export const HAMLoading = () => {
  return (
    <Flex direction={'column'} gap={8} maxH={"full"} overflowY={"hidden"}>
      <Flex gap={5}>
        <Skeleton height={16} width="full" rounded={"xl"} />
        <Skeleton height={16} width={20} rounded={"xl"} />
      </Flex>

      <Flex direction={'column'} gap={5}>
        <Skeleton height={20} width="full" rounded={"xl"} />
        <Flex direction={'column'} gap={5}>
          {Array.from({ length: 3 }).map((_, index) => (
            <Skeleton key={index} height={12} width="full" rounded={"xl"} />
          ))}
        </Flex>
      </Flex>
      <Flex justify={'space-between'} align={'center'}>
        <Skeleton height={14} width={"sm"} rounded={"xl"} />
        <Skeleton height={14} width={"sm"} rounded={"xl"} />
      </Flex>
    </Flex>
  )
}

export const HAMDetailLoading = () => {
  return (
    <Flex
      w={'sm'}
      bg={'primary.100'}
      transition={'all 0.3s'}
      direction={'column'}
      p={7}
      gap={8}
      h={"fit"}
      overflowY={'auto'}
    >
      <Flex direction={'column'} gap={8} flex={1}>
        <Skeleton height={28} width="full" rounded={"xl"} />

        <Flex direction={'column'} gap={5} flex={1}>
          {Array.from({ length: 2 }).map((_, index) => (
            <Skeleton key={index} height={14} width="full" rounded={"xl"} />
          ))}
        </Flex>
        <Flex direction={'column'} gap={5}>
          {Array.from({ length: 1 }).map((_, index) => (
            <Skeleton key={index} height={16} width="full" rounded={"xl"} />
          ))}
        </Flex>
      </Flex>
    </Flex>
  )
}