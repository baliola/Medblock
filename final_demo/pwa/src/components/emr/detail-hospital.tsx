import { isValidElement } from "react";
import { Badge, Button, Divider, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, ModalOverlay, Stack, Text } from "@chakra-ui/react";
import { GetProviderBatchResponse } from "@/declarations/provider_registry/provider_registry.did";
import { convertBigIntToTime } from "@/utils/format-time";
import { emrHospitalInfo } from "@/constants/contents/emr/detail/hospital-info";

interface DetailHospitalInfoProps {
  isOpen: boolean;
  onClose: () => void;
  hospital: GetProviderBatchResponse
}

enum HospitalStatus {
  Active = 'Active',
  Suspended = 'Suspended',
  Unknown = 'Unknown'
}

interface HospitalInfoFieldProps {
  label: string;
  value: string | number | React.ReactNode;
}

const HospitalInfoField = ({ label, value }: HospitalInfoFieldProps) => (
  <Stack spacing={0}>
    <Text color={'gray.500'} fontSize={'sm'}>
      {label}
    </Text>
    {isValidElement(value)
      ? value
      : (
        <Text fontSize={'md'} fontWeight={'bold'}>
          {value}
        </Text>
      )
    }
  </Stack>
);

const DisplayStatus = ({ status }: { status: Record<string, null> }) => {
  const statusKey = Object.keys(status)[0];
  const badgeStatus = emrHospitalInfo.status;

  switch (statusKey) {
    case HospitalStatus.Active:
      return (
        <Badge colorScheme={badgeStatus.active.colorScheme} w="fit-content" px={2} rounded="md">
          {badgeStatus.active.label}
        </Badge>
      );
    case HospitalStatus.Suspended:
      return (
        <Badge colorScheme={badgeStatus.suspended.colorScheme} w="fit-content" px={2} rounded="md">
          {badgeStatus.suspended.label}
        </Badge>
      );
    default:
      return (
        <Badge colorScheme={badgeStatus.unknown.colorScheme} w="fit-content" px={2} rounded="md">
          {badgeStatus.unknown.label}
        </Badge>
      );
  }
}

export default function DetailHospitalInfo({
  isOpen, onClose, hospital
}: DetailHospitalInfoProps) {
  if (!hospital) return null;

  const { header, contents, footer } = emrHospitalInfo;
  const providerData = hospital.providers[0].V1;

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      size={{ base: 'full', md: 'md' }}
    >
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>
          {header.title}
        </ModalHeader>
        <ModalBody>
          <Stack divider={<Divider />} spacing={3}>
            <HospitalInfoField label={contents.hospital_id.label} value={providerData.internal_id} />
            <HospitalInfoField label={contents.hospital_name.label} value={providerData.display_name} />
            <HospitalInfoField label={contents.hospital_address.label} value={providerData.address} />
            <HospitalInfoField label={contents.registered_at.label} value={convertBigIntToTime(providerData.registered_at)} />
            <Stack spacing={1}>
              <HospitalInfoField label={contents.hospital_status.label}
                value={
                  <DisplayStatus status={providerData.activation_status} />
                }
              />
            </Stack>
          </Stack>
        </ModalBody>
        <ModalFooter pt={8}>
          <Button w={'full'} colorScheme="primary" bg={'primary.700'} onClick={onClose}>
            {footer.button.label}
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  )
}