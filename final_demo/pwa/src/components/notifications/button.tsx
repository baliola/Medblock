"use client"

import { Fragment, useState } from "react";
import { Button, Icon, Text, useDisclosure } from "@chakra-ui/react";
import { IoNotifications } from "react-icons/io5";
import Notifications from "@/components/notifications";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { useNotificationStore } from "@/store/notifications-store";
import { patientCanisterId } from "@/config/canisters/patient.canister";

const NotificationButton = () => {
  const [notificationLength, setNotificationLength] = useState<number>(0);
  const setError = useNotificationStore(state => state.setError);
  const setLoading = useNotificationStore(state => state.setLoading);
  const setNotification = useNotificationStore(state => state.setNotification);
  const setIsError = useNotificationStore(state => state.setIsError);

  const { isOpen, onOpen, onClose } = useDisclosure();

  const { call: refetchLogs, data } = usePatientQuery({
    functionName: "get_logs",
    onSuccess(data) {
      console.log(data);

      // @ts-expect-error
      setNotificationLength(data?.logs.length);

      // @ts-expect-error
      setNotification(data)
    },
    onError(error) {
      setIsError(true);
      setError(error as any)
    },
    onLoading(loading) {
      setLoading(loading)
    },
  })

  const onNotification = async () => {
    onOpen();
    await refetchLogs()
  }

  return (
    <Fragment>
      <Notifications isOpen={isOpen} onClose={onClose} />
      <Button
        variant={'ghost'}
        w={'fit-content'}
        p={0}
        onClick={onNotification}
        pos={'relative'}
      >
        <Icon
          as={IoNotifications}
          color={'warning.600'}
          boxSize={7}
        />

        {notificationLength > 0 && (
          <Text
            pos={'absolute'}
            top={2}
            right={2}
            fontSize={'10px'}
            color={'white'}
            bg={'red.500'}
            rounded={'full'}
            px={1} py={'0.5px'}
          >
            {notificationLength}
          </Text>
        )}
      </Button>
    </Fragment>
  )
}

export default function ButtonNotifications() {
  return (
    <PatientActor
      canisterId={patientCanisterId}
    >
      <NotificationButton />
    </PatientActor>
  )
}