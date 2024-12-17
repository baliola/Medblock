import {
  Button,
  Drawer,
  DrawerBody,
  DrawerContent,
  DrawerOverlay,
  Icon,
  Text,
  useDisclosure,
} from "@chakra-ui/react";
import { BsThreeDotsVertical } from "react-icons/bs";
import { HiOutlineEye } from "react-icons/hi2";
import LeaveGroupModal from "./leave";
import { useProfileStore } from "@/store/profile-store";
import GrantAccessGroupModal from "./grant";
import RevokeAccessGroupModal from "./revoke";
import { usePatientQuery } from "@/services/patients";
import { EmrListPatientResponse } from "@/declarations/patient_registry/patient_registry.did";
import { useState } from "react";
import EMRListModal from "../modal-emr/emr-list";
import { useRouter } from "next/navigation";

interface DetailModalProps {
  isLeader: boolean;
  nik: string;
  group_id: string;
  name: string
}

export default function DetailModal({ props }: { props: DetailModalProps }) {
  const { isLeader, nik, group_id, name } = props;
  const router = useRouter()

  const { isOpen, onOpen, onClose } = useDisclosure();
  const { profile } = useProfileStore();

  const [emrGroupInformation, setEmrGroupInformation] = useState<
    EmrListPatientResponse | null | undefined
  >(undefined);

  const {
    call: getEmrGroupInformation,
    loading: loadingGetEmrGroupInformation,
  } = usePatientQuery({
    functionName: "view_group_member_emr_information",
    refetchOnMount: false,
    onSuccess(data) {
      console.log(data);
      const { Ok }: any = data;
      if (Ok) setEmrGroupInformation(Ok);
      else setEmrGroupInformation(null);
    },
    onError(error) {
      setEmrGroupInformation(null);
      console.error(error);
    },
  });

  return (
    <>
      <Button type="button" onClick={onOpen}>
        <BsThreeDotsVertical />
      </Button>
      <Drawer isOpen={isOpen} placement="bottom" onClose={onClose}>
        <DrawerOverlay />
        <DrawerContent>
          <DrawerBody
            textAlign={"center"}
            display={"flex"}
            flexDirection={"column"}
            bg={"#EFF0FC"}
            py={6}
            px={6}
            rowGap={3}
          >
            {nik !== profile?.nik && (
              <>
                <GrantAccessGroupModal props={{ nik }} />
                <RevokeAccessGroupModal props={{ nik }} />
              </>
            )}

            {nik !== profile?.nik ? (
              <>
                <EMRListModal props={{ group_id, nik, name }} />
              </>
            ) : (
              <Button
                type="button"
                bg={"transparent"}
                display={"flex"}
                justifyContent={"items-start"}
                columnGap={3}
                fontWeight={400}
                color={"primary.700"}
                onClick={() => { router.push('/home') }}
                leftIcon={<Icon as={HiOutlineEye} boxSize={6} />}
              >
                <Text>See Group EMRs</Text>
              </Button>
            )}

            {nik === profile?.nik && <LeaveGroupModal props={{ isLeader }} />}
          </DrawerBody>
        </DrawerContent>
      </Drawer>
    </>
  );
}
