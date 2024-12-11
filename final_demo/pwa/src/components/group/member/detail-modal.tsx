import { Button, Drawer, DrawerBody, DrawerContent, DrawerOverlay, Icon, Text, useDisclosure } from "@chakra-ui/react";
import { BsThreeDotsVertical } from "react-icons/bs";
import { HiOutlineEye } from "react-icons/hi2";
import LeaveGroupModal from "./leave";
import { useProfileStore } from "@/store/profile-store";
import GrantAccessGroupModal from "./grant";
import RevokeAccessGroupModal from "./revoke";

interface DetailModalProps {
  isLeader: boolean
  nik: string
}

export default function DetailModal({ props }: { props: DetailModalProps}) {
  const { isLeader, nik } = props
  const { isOpen, onOpen, onClose } = useDisclosure()
  const { profile } = useProfileStore()

  return (
    <>
      <Button
        type="button"
        onClick={onOpen}
      >
        <BsThreeDotsVertical />
      </Button>
      <Drawer
        isOpen={isOpen}
        placement="bottom"
        onClose={onClose}
      >
        <DrawerOverlay />
        <DrawerContent>
          <DrawerBody 
            textAlign={'center'}
            display={"flex"}
            flexDirection={"column"}
            bg={"#EFF0FC"}
            py={6}
            px={6}
            rowGap={3}
          >
            {
              nik !== profile?.nik &&
              <>
                <GrantAccessGroupModal props={{ nik }} />
                <RevokeAccessGroupModal props={{ nik }} />
              </>
            }

            <Button
              type="button"
              bg={"transparent"}
              display={"flex"}
              justifyContent={"items-start"}
              columnGap={3}
              fontWeight={400}
              color={"primary.700"}
              leftIcon={
                <Icon as={HiOutlineEye} boxSize={6}  />
              }
            >
              <Text>
                See Group EMRs
              </Text>
            </Button>
            {
              nik === profile?.nik &&
              <LeaveGroupModal props={{ isLeader }} />
            }
          </DrawerBody>
        </DrawerContent>
      </Drawer>
    </>
  )
}