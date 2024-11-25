"use client"

import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { Button, Flex, Icon, Text, useDisclosure, useToast } from "@chakra-ui/react";
import { FaX } from "react-icons/fa6";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { getUamDetail, updateUam } from "@/libs/api/uam";

import { uamDetailButton, uamDetailHeader, uamDetailProfile } from "@/constants/contents/uam/detail";
import HorizontalProfile from "@/components/profile/horizontal";
import UAMDeniedModal from "@/components/uam/detail/denied";
import { UAMDetailLoading } from "@/components/uam/loading";
import IDCardView from "./id-card";

interface InfoRowProps {
  icon: React.ElementType;
  title: string;
  value: string;
}

const InfoRow = ({
  icon,
  title,
  value
}: InfoRowProps) => {
  return (
    <Flex align={'center'} gap={5}>
      <Icon as={icon} boxSize={8} color={"neutral.700"} />
      <Flex direction={'column'} color={"neutral.700"}>
        <Text>{title}</Text>
        <Text fontSize={'lg'} fontWeight={'bold'}>{value}</Text>
      </Flex>
    </Flex>
  )
}

const IDCardInfo = ({ data }: { data: string }) => {
  const {
    isOpen: isIDCardOpen,
    onOpen: onIDCardOpen,
    onClose: onIDCardClose
  } = useDisclosure();

  return (
    <Flex align={'center'} gap={5}>
      <Icon as={uamDetailProfile.identity_card.icon} boxSize={8} color={"neutral.700"} />
      <Flex direction={'column'} color={"neutral.700"} gap={3}>
        <Text>{uamDetailProfile.identity_card.label}</Text>
        <Button
          colorScheme={'primary'}
          bg={"primary.700"}
          size={'sm'}
          onClick={onIDCardOpen}
        >
          View ID Card
        </Button>
        <IDCardView
          isOpen={isIDCardOpen}
          onCLose={onIDCardClose}
          data={data}
        />
      </Flex>
    </Flex>
  )
}

export default function UAMDetail() {
  const toast = useToast();

  const router = useRouter();
  const pathname = usePathname();
  const params = useSearchParams();

  const user = params.get('user');
  const page = params.get('page') || "1";
  const limit = params.get('limit') || "10";

  const { isOpen, onOpen, onClose } = useDisclosure();

  const queryClient = useQueryClient();

  const { data, isLoading } = useQuery({
    queryKey: ['uam', { user }],
    queryFn: () => getUamDetail({ user: user as string }),
    enabled: !!user,
    refetchOnWindowFocus: false,
    refetchInterval: 5 * 60 * 1000 // 5 minutes
  });

  const {
    mutate: approveUser,
    isPending
  } = useMutation({
    mutationKey: ['uam', 'approved', { user }],
    mutationFn: () => updateUam({
      user: user as string,
      verification: 'accepted'
    }),
    onSuccess() {
      queryClient.invalidateQueries({
        queryKey: ['uam', { user }]
      });

      queryClient.invalidateQueries({
        queryKey: ['uam', { page, limit }]
      });

      return toast({
        title: "User approved",
        description: "User has been approved",
        status: "success",
      })
    },
    onError(error) {
      console.error(error);

      return toast({
        title: 'Error',
        description: 'Failed to approve user.',
        status: 'error',
      })
    }
  })

  const onCloseDetail = () => {
    const param = new URLSearchParams(params);
    param.delete('user');

    const path = `${pathname}?${param.toString()}`;
    router.push(path);
  }

  if (isLoading) return <UAMDetailLoading />
  if (!data) return;

  return (
    <Flex
      w={'xl'}
      bg={'primary.100'}
      transition={'all 0.3s'}
      direction={'column'}
      p={7}
      gap={8}
      maxH={'100dvh'}
      overflowY={'auto'}
    >
      <UAMDeniedModal isOpen={isOpen} onClose={onClose} />

      <Flex align={'center'} gap={5}>
        <Button size={'xs'} rounded={'full'} colorScheme="red" p={0}>
          <Icon as={FaX} onClick={onCloseDetail} />
        </Button>
        <Text fontSize={'2xl'} fontWeight={'bold'}>
          {uamDetailHeader.title}
        </Text>
      </Flex>

      <Flex direction={"column"} gap={8} flex={1}>
        <HorizontalProfile profile={data} />
        <Flex direction={'column'} gap={5} ps={4}>
          <Text fontSize={'lg'} fontWeight={'bold'} color={"neutral.700"}>
            {uamDetailProfile.header.title}
          </Text>
          <InfoRow
            icon={uamDetailProfile.full_name.icon}
            title={uamDetailProfile.full_name.label}
            value={data?.fullName}
          />
          <InfoRow
            icon={uamDetailProfile.address.icon}
            title={uamDetailProfile.address.label}
            value={data?.address}
          />
          <InfoRow
            icon={uamDetailProfile.place_of_birth.icon}
            title={uamDetailProfile.place_of_birth.label}
            value={data?.placeBirth}
          />
          <InfoRow
            icon={uamDetailProfile.birthdate.icon}
            title={uamDetailProfile.birthdate.label}
            value={data?.dateBirth}
          />
          <InfoRow
            icon={uamDetailProfile.martial_status.icon}
            title={uamDetailProfile.martial_status.label}
            value={data?.marital}
          />
          <IDCardInfo data={data?.idCard} />
        </Flex>
      </Flex>

      <Flex direction={'column'} gap={3}>
        <Button
          colorScheme="accent"
          variant={'outline'}
          w="full"
          size="lg"
          rounded="2xl"
          _hover={{ bg: 'accent.100' }}
          onClick={onOpen}
          isDisabled={isPending}
        >
          {uamDetailButton.denied.label}
        </Button>
        {data.verification !== 'accepted' && (
          <Button
            colorScheme="primary"
            bg="primary.700"
            w="full"
            size="lg"
            rounded="2xl"
            onClick={() => approveUser()}
            isLoading={isPending}
          >
            {uamDetailButton.approved.label}
          </Button>
        )}
      </Flex>
    </Flex>
  )
}