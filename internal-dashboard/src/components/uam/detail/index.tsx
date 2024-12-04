"use client"

import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { Button, Flex, Icon, Text, useDisclosure, useToast } from "@chakra-ui/react";
import { FaX } from "react-icons/fa6";
// import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
// import { getUamDetail, updateUam } from "@/libs/api/uam";

import { uamDetailButton, uamDetailHeader, uamDetailProfile } from "@/constants/contents/uam/detail";
import HorizontalProfile from "@/components/profile/horizontal";
import UAMDeniedModal from "@/components/uam/detail/denied";
import { UAMDetailLoading } from "@/components/uam/loading";
import IDCardView from "./id-card";
import { usePatientQuery } from "@/services/patients";
import { usePatientStore } from "@/store/patients.store";
import { KycStatus, PatientListAdminResponse, UpdateKycStatusRequest } from "@/canister/declarations/patient_registry/patient_registry.did";
import { FiCheckCircle } from "react-icons/fi";
// import { useUserPrincipal } from "@ic-reactor/react";

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

const PatientDetail = ({ getPatientList }: { getPatientList: () => Promise<PatientListAdminResponse | undefined>}) => {
  const toast = useToast();

  const router = useRouter();
  const pathname = usePathname();
  const params = useSearchParams();

  const user = params.get('user');
  // const page = params.get('page') || "1";
  // const limit = params.get('limit') || "10";

  const { isOpen, onClose } = useDisclosure(); 

  // const queryClient = useQueryClient();

  // const { data, isLoading } = useQuery({
  //   queryKey: ['uam', { user }],
  //   queryFn: () => getUamDetail({ user: user as string }),
  //   enabled: !!user,
  //   refetchOnWindowFocus: false,
  //   refetchInterval: 5 * 60 * 1000 // 5 minutes
  // });

  // const {
  //   mutate: approveUser,
  //   isPending
  // } = useMutation({
  //   mutationKey: ['uam', 'approved', { user }],
  //   mutationFn: () => updateUam({
  //     user: user as string,
  //     verification: 'accepted'
  //   }),
  //   onSuccess() {
  //     queryClient.invalidateQueries({
  //       queryKey: ['uam', { user }]
  //     });

  //     queryClient.invalidateQueries({
  //       queryKey: ['uam', { page, limit }]
  //     });

  //     return toast({
  //       title: "User approved",
  //       description: "User has been approved",
  //       status: "success",
  //     })
  //   },
  //   onError(error) {
  //     console.error(error);

  //     return toast({
  //       title: 'Error',
  //       description: 'Failed to approve user.',
  //       status: 'error',
  //     })
  //   }
  // })

  const { patient, setPatient } = usePatientStore();
  // const principal = useUserPrincipal()

  const { call: searchPatient, loading: loadingSearchPatient } = usePatientQuery({
    functionName: "search_patient_admin",
    args: [{ nik: user ?? '' }],
    refetchOnMount: true,
    onSuccess(data) {
      const result = data?.patient_info
      setPatient(result);
    },
    onError(error) {
      setPatient(null)
      console.error(user)
      console.error(error);
    },
  });

  const onCloseDetail = () => {
    const param = new URLSearchParams(params);
    param.delete('user');

    const path = `${pathname}?${param.toString()}`;
    router.push(path);
  }

  const { call: updateKycStatus, loading: loadingUpdateKycStatus } = usePatientQuery({
    functionName: "update_kyc_status",
    refetchOnMount: false,
    onSuccess() {
      getPatientList()
      return
    },
    onError(error) {
      throw error;
    },
  });

  const handleUpdateKycStatus = async (nik: string, kyc_status: KycStatus) => {
    try {
      const data: UpdateKycStatusRequest = {
        nik,
        kyc_status,
      };

      await updateKycStatus([data]);
      searchPatient([{ nik }])

      toast({
        title: Object.keys(kyc_status)[0].toLowerCase() === 'approved' ? 'KYC Approved' : 'KYC Denied',
        description: Object.keys(kyc_status)[0].toLowerCase() === 'approved' ? 'Successfully aprrove KYC fot this user' : 'Successfully deny KYC fot this user',
        status: "success",
      });
    } catch (error: unknown) {
      if (error instanceof Error) {
        toast({
          title: error.name,
          description: error.message,
          status: "error",
        });
      }

      console.error(error)
    }
  };

  if (loadingSearchPatient) return <UAMDetailLoading />
  if (!patient) return;

  return (
    <Flex
      w={'24rem'}
      bg={'primary.100'}
      transition={'all 0.3s'}
      direction={'column'}
      p={7}
      gap={8}
      maxH="100vh"
      overflowY={'auto'}
      css={{
        "&::-webkit-scrollbar": { display: "none" }, // Chrome, Safari
        "-msOverflowStyle": "none", // IE and Edge
        "scrollbarWidth": "none", // Firefox
      }}
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
        <HorizontalProfile profile={patient} />
        <Flex direction={'column'} gap={5} ps={4} textTransform={"capitalize"}>
          <Text fontSize={'lg'} fontWeight={'bold'} color={"neutral.700"}>
            {uamDetailProfile.header.title}
          </Text>
          <InfoRow
            icon={uamDetailProfile.full_name.icon}
            title={uamDetailProfile.full_name.label}
            value={patient.info.V1.name}
          />
          <InfoRow
            icon={uamDetailProfile.address.icon}
            title={uamDetailProfile.address.label}
            value={patient.info.V1.address}
          />
          <InfoRow
            icon={uamDetailProfile.place_of_birth.icon}
            title={uamDetailProfile.place_of_birth.label}
            value={patient.info.V1.place_of_birth}
          />
          <InfoRow
            icon={uamDetailProfile.birthdate.icon}
            title={uamDetailProfile.birthdate.label}
            value={patient.info.V1.date_of_birth}
          />
          <InfoRow
            icon={uamDetailProfile.martial_status.icon}
            title={uamDetailProfile.martial_status.label}
            value={patient.info.V1.martial_status}
          />
          {
            Object.keys(patient.info.V1.kyc_status)[0].toLowerCase() !== 'pending' &&
            <Flex align={'center'} gap={5}>
              <Icon as={FiCheckCircle} boxSize={8} color={"neutral.700"} />
              <Flex direction={'column'} color={"neutral.700"}>
                <Text>Status</Text>
                <Text 
                  fontSize={'lg'} 
                  fontWeight={'bold'} 
                  textColor={Object.keys(patient.info.V1.kyc_status)[0].toLowerCase() === 'approved' ? 'green' : 'red'}
                >
                  {Object.keys(patient.info.V1.kyc_status)[0]}
                </Text>
              </Flex>
            </Flex>
          }
          <IDCardInfo data={patient.nik} />
        </Flex>
      </Flex>

      {
        Object.keys(patient.info.V1.kyc_status)[0].toLowerCase() === 'pending' 
          && <Flex direction={'column'} gap={3}>
            <Button
              colorScheme="accent"
              variant={'outline'}
              w="full"
              size="lg"
              rounded="2xl"
              _hover={{ bg: 'accent.100' }}
              onClick={() => {
                handleUpdateKycStatus(patient.nik, { Denied: null })
              }}
              isLoading={loadingUpdateKycStatus}
              // isDisabled={isPending}
            >
              {uamDetailButton.denied.label}
            </Button>
            <Button
              colorScheme="primary"
              bg="primary.700"
              w="full"
              size="lg"
              rounded="2xl"
              onClick={() => {
                handleUpdateKycStatus(patient.nik, { Approved: null })
              }}
              isLoading={loadingUpdateKycStatus}
              // isLoading={isPending}
            >
              {uamDetailButton.approved.label}
            </Button>
          </Flex>
      }

    </Flex>
  )
}

export default function UAMDetail({ getPatientList }: { getPatientList: () => Promise<PatientListAdminResponse | undefined>}) {
  return (
    <PatientDetail getPatientList={getPatientList}/>
  );
}