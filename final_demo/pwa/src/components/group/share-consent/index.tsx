"use client"

import { Fragment, useEffect, useState } from "react";
import { Button, ButtonProps, Flex, useDisclosure } from "@chakra-ui/react";
import { PatientActor, usePatientQuery } from "@/services/patients";

import LoadingScreen from "@/layouts/loading";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import { useUserPrincipal } from "@ic-reactor/react";
import { useRouter } from "next/navigation";
import { CreateConsentForGroupRequest, CreateConsentForGroupResponse } from "@/declarations/patient_registry/patient_registry.did";
import { useProfileStore } from "@/store/profile-store";
import ModalInputPin from "@/components/share-concent/input-pin";
import PinShareConcentChecker from "@/components/share-concent/pin-checker";
import ConcentCodeView from "./consent-code";

const ButtonShareConcentCodeGroup = ({ children, ...props }: ButtonProps) => {
  const router = useRouter();
  const { isOpen, onOpen, onClose } = useDisclosure();
  const { profile } = useProfileStore()

  const [isModalOpen, setIsModalOpen] = useState<boolean>(false);
  const [modalInputPin, setModalInputPin] = useState<boolean>(false);
  const [userHasPin, setUserHasPin] = useState<boolean>(false);
  const [consentCode, setConsentCode] = useState<string | null>(null)

  const principal = useUserPrincipal();

  const {
    call: fetchConcent,
    data,
    loading: concentLoading,
    error
  } = usePatientQuery({
    functionName: "create_consent_for_group",
    refetchOnMount: false,
    onSuccess(data: CreateConsentForGroupResponse | undefined) {

      if (data) {
        setConsentCode(data.group_consent_code)
        onOpen();
      }
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

    if (profile) {
      const data: CreateConsentForGroupRequest[] | any | undefined = [{
        nik: profile.nik,
      }];

      fetchConcent(data);
    }
  }

  const onSubmitPinChecker = () => {
    router.push("/pin/add")
  }

  const onSuccessInputPin = () => {
    setModalInputPin(false);

    if (profile) {
      const data: CreateConsentForGroupRequest[] | any | undefined = [{
        nik: profile.nik,
      }];

      fetchConcent(data);
    }
  }

  return (
    <Fragment>
      <ModalInputPin
        isOpen={modalInputPin}
        onClose={() => setModalInputPin(false)}
        onSuccess={onSuccessInputPin}
      />

      {
        consentCode &&
        <ConcentCodeView
          isOpen={isOpen}
          onClose={onClose}
          code={consentCode}
          refetchCode={() => {
            if (profile) {
              const data: CreateConsentForGroupRequest[] | any | undefined = [{
                nik: profile.nik,
              }];
        
              fetchConcent(data);
            }
          }}
          isFetching={concentLoading}
        />
      }

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

export default function ShareConcentCodeGroup({ children, ...props }: ButtonProps) {
  return (
    <PatientActor
      canisterId={patientCanisterId}
      loadingComponent={<LoadingScreen />}
    >
      <ButtonShareConcentCodeGroup {...props}>
        {children}
      </ButtonShareConcentCodeGroup>
    </PatientActor>
  )
}