"use client"

import { Fragment, useEffect, useState } from "react";
import { Button, ButtonProps, useDisclosure } from "@chakra-ui/react";
import { PatientActor, usePatientQuery } from "@/services/patients";

import LoadingScreen from "@/layouts/loading";
import ConcentCodeView from "@/components/share-concent/concent-code";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import { useUserPrincipal } from "@ic-reactor/react";
import PinShareConcentChecker from "./pin-checker";
import ModalInputPin from "./input-pin";
import { useRouter } from "next/navigation";

const ButtonShareConcentCode = ({ children, ...props }: ButtonProps) => {
  const router = useRouter();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const [isModalOpen, setIsModalOpen] = useState<boolean>(false);
  const [modalInputPin, setModalInputPin] = useState<boolean>(false);
  const [userHasPin, setUserHasPin] = useState<boolean>(false);

  const principal = useUserPrincipal();

  const {
    call: fetchConcent,
    data,
    loading: concentLoading,
    error
  } = usePatientQuery({
    functionName: "create_consent",
    refetchOnMount: false,
    onSuccess() {
      onOpen();
    },
    onError(error) {
      console.log(error)
    },
  });

  useEffect(() => {
    if (!principal) return;

    const localPin = localStorage.getItem(`pin@${principal.toText()}`);
    console.log(localPin, principal.toText());

    if (localPin) {
      setUserHasPin(true);
    }

  }, [principal]);

  const handleFetchConcent = () => {
    userHasPin
      ? setModalInputPin(true)
      : setIsModalOpen(true);
  }

  const onClosePinChecker = () => {
    setIsModalOpen(false);
    fetchConcent();
  }

  const onSubmitPinChecker = () => {
    router.push("/pin/add")
  }

  const onSuccessInputPin = () => {
    setModalInputPin(false);
    fetchConcent();
  }

  return (
    <Fragment>
      <ModalInputPin
        isOpen={modalInputPin}
        onClose={() => setModalInputPin(false)}
        onSuccess={onSuccessInputPin}
      />

      <ConcentCodeView
        isOpen={isOpen}
        onClose={onClose}
        // @ts-ignore
        code={data?.code}
        refetchCode={() => fetchConcent()}
        isFetching={concentLoading}
      />

      <PinShareConcentChecker
        isOpen={isModalOpen}
        onClose={onClosePinChecker}
        onSubmit={onSubmitPinChecker}
      />

      <Button type="submit"
        colorScheme="primary"
        bg={"primary.700"}
        fontSize={'sm'}
        w={'full'}
        py={6}
        rounded={"xl"}
        {...props}
        onClick={handleFetchConcent}
        isLoading={concentLoading}
      >
        {children}
      </Button>
    </Fragment>
  )
}

export default function ShareConcentCode({ children, ...props }: ButtonProps) {
  return (
    <PatientActor
      canisterId={patientCanisterId}
      loadingComponent={<LoadingScreen />}
    >
      <ButtonShareConcentCode {...props}>
        {children}
      </ButtonShareConcentCode>
    </PatientActor>
  )
}