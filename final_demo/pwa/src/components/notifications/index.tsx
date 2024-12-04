"use client"

import { notificationActivity, notificationsHeader } from "@/constants/contents/notifications";
import { Activity, ActivityType } from "@/declarations/patient_registry/patient_registry.did";
import { useNotificationStore } from "@/store/notifications-store";
import { convertBigIntToTime } from "@/utils/format-time";
import { Button, Divider, Drawer, DrawerBody, DrawerContent, DrawerHeader, DrawerOverlay, Flex, Icon, Stack, Text } from "@chakra-ui/react";
import { FaCheckCircle } from "react-icons/fa";
import { FaChevronLeft, FaCircleXmark, FaFileArrowUp } from "react-icons/fa6";
import { IoDocumentText } from "react-icons/io5";

enum ActivityTypes {
  Updated = 'Updated',
  Accessed = 'Accessed',
  Revoked = 'Revoked',
}

const DisplayNotification = ({ log }: { log: Activity }) => {
  function getActivityText(activity: ActivityType): string {
    const { updated, accessed, revoked, error } = notificationActivity;

    switch (Object.keys(activity)[0]) {
      case ActivityTypes.Updated:
        return updated.label;
      case ActivityTypes.Accessed:
        return accessed.label;
      case ActivityTypes.Revoked:
        return revoked.label;
      default:
        throw new Error(error.label);
    }
  }

  const IconLogType = ({ log }: { log: ActivityType }) => {
    switch (Object.keys(log)[0]) {
      case ActivityTypes.Updated:
        return <Icon as={FaFileArrowUp} boxSize={5} color={'blue.500'} />;
      case ActivityTypes.Accessed:
        return <Icon as={FaCheckCircle} boxSize={5} color={'green.500'} />;
      case ActivityTypes.Revoked:
        return <Icon as={FaCircleXmark} boxSize={5} color={'red.500'} />;
      default:
        return <Icon as={IoDocumentText} boxSize={5} color={'blue.500'} />;
    }
  }

  return (
    <Flex align={'start'} gap={5}>
      <Flex mt={1}>
        <IconLogType log={log.activity_type} />
      </Flex>
      <Flex direction={'column'} gap={1}>
        <Text fontSize={'md'} fontWeight={'bold'}>
          {getActivityText(log.activity_type)}
        </Text>
        <Text fontSize={'sm'}>
          {convertBigIntToTime(log.timestamp)}
        </Text>
      </Flex>
    </Flex>
  )
}

export default function Notifications({
  isOpen,
  onClose
}: {
  isOpen: boolean;
  onClose: () => void;
}) {
  const notifications = useNotificationStore(state => state.notification);
  const error = useNotificationStore(state => state.error)
  const isError = useNotificationStore(state => state.isError)
  const loading = useNotificationStore(state => state.loading)

  return (
    <Drawer
      isOpen={isOpen}
      onClose={onClose}
      size={"full"}
      placement="right"
    >
      <DrawerOverlay>
        <DrawerContent>
          <DrawerHeader px={3}>
            <Flex align={'center'} w={'fit-content'} onClick={onClose}>
              <Button
                w={'fit-content'}
                p={0}
                variant={'unstyled'}
                alignItems={'center'}
                display={'flex'}
              >
                <Icon as={FaChevronLeft} />
              </Button>
              {notificationsHeader.title}
            </Flex>
          </DrawerHeader>
          <DrawerBody>
            <Stack spacing={3} divider={<Divider />} pb={5}>
              {notifications?.logs
                .slice()
                .reverse()
                .map((log, index) => (
                  <DisplayNotification key={index} log={log} />
                ))}
            </Stack>
          </DrawerBody>
        </DrawerContent>
      </DrawerOverlay>
    </Drawer>
  )
}