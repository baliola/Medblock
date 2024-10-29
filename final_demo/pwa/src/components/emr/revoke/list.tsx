"use client"

import { useEffect } from "react";
import { Button, Checkbox, Flex, Icon, Skeleton, SlideFade, Text, useToast } from "@chakra-ui/react";
import { FaHospital } from "react-icons/fa6";

import { ProviderActor, useProviderQuery } from "@/services/providers";
import { usePatientQuery } from "@/services/patients";
import { providerCanisterId } from "@/config/canisters/providers.canister";
import { Consent, RevokeConsentRequest } from "@/declarations/patient_registry/patient_registry.did";
import { GetProviderBatchRequest } from "@/declarations/provider_registry/provider_registry.did";

import EmptyHistoryEMR from "@/components/home/no-history";
import RevokeConfirmationAlert from "@/components/emr/revoke/alert";
import { useConsentStore } from "@/store/consent-store";
import { emrRevokeButton } from "@/constants/contents/emr/revoke/button";
import { emrRevokeToast } from "@/constants/contents/emr/revoke/toast";

const ConcentList = () => {
  const toast = useToast();

  const listUserConsent = useConsentStore(state => state.listUserConsent);
  const setListUserConsent = useConsentStore(state => state.setListUserConsent);
  const revokeConsent = useConsentStore(state => state.revokeConsent);
  const setRevokeConsent = useConsentStore(state => state.setRevokeConsent);
  const isOpenConfirmation = useConsentStore(state => state.isOpenConfirmation);
  const onOpenConfirmation = useConsentStore(state => state.onOpenConfirmation);
  const onCloseConfirmation = useConsentStore(state => state.onCloseConfirmation);

  const {
    call: revokeAccessByCode,
    loading: loadingRevokeAccessByCode
  } = usePatientQuery({
    functionName: "revoke_consent",
    refetchOnMount: false,
  });

  const {
    call: getProvider,
    loading: loadingProvider
  } = useProviderQuery({
    functionName: "get_provider_batch",
    refetchOnMount: false,
  })

  const {
    call: getConcentList,
    loading: loadingConcentList
  } = usePatientQuery({
    functionName: "consent_list",
    refetchOnMount: false,
    onSuccess(data) {
      // @ts-expect-error
      const consentList: Array<Consent> = data?.consents;

      // @ts-expect-error
      const session_user: string[] = data?.consents.map((consent: Consent) => {
        return consent.claimed
          ? consent.session_user
          : []
      }).flat();

      const request: GetProviderBatchRequest = {
        ids: session_user
      };

      // @ts-expect-error
      getProvider([request])
        .then((data) => {
          // @ts-expect-error
          const mergedData = data?.providers.map((provider, index) => {
            return {
              ...provider,
              session_user: session_user[index],
              // @ts-expect-error
              code: consentList.find((consent) => consent.session_user.includes(session_user[index]))?.code
            }
          })

          setListUserConsent(mergedData)
        })
        .catch((error) => {
          console.log(error)
        })
    },
    onError(error) {
      console.log(error)
    }
  });

  useEffect(() => {
    (async () => {
      await getConcentList();
    })();

    // eslint-disable-next-line
  }, [])

  const onCheckboxChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { checked, value } = event.target;

    if (checked) {
      setRevokeConsent([...revokeConsent, value])
    } else {
      setRevokeConsent(revokeConsent.filter((consent) => consent !== value))
    }
  }

  const onRevoke = async () => {
    const { error, success } = emrRevokeToast;

    const request: RevokeConsentRequest = {
      codes: revokeConsent
    };

    try {
      // @ts-expect-error
      await revokeAccessByCode([request])

      setListUserConsent(
        listUserConsent?.filter((consent) => !revokeConsent.includes(consent.code))
      )
      setRevokeConsent(
        revokeConsent.filter((consent) => !revokeConsent.includes(consent))
      )

      return toast({
        title: success.title,
        description: success.description,
        status: "success",
        duration: 5000,
        isClosable: true,
        position: "top-right"
      });
    } catch (err) {
      return toast({
        title: error.title,
        description: error.description,
        status: "error",
        duration: 5000,
        isClosable: true,
        position: "top-right"
      });
    } finally {
      onCloseConfirmation();
    }
  }

  if (
    loadingConcentList ||
    loadingProvider
  ) {
    return (
      <Flex direction={'column'} gap={5} w={'full'}>
        {Array.from({ length: 3 }).map((_, index) => (
          <Skeleton key={index} height={24} w={'full'} rounded={'xl'} />
        ))}
      </Flex>
    )
  }

  if (
    listUserConsent &&
    listUserConsent.length < 1
  ) {
    return (
      <EmptyHistoryEMR refreshData={getConcentList} />
    )
  }

  return (
    <Flex
      direction={'column'}
      gap={5}
      flex={1}
      justify={'space-between'}
    >
      <RevokeConfirmationAlert
        isOpen={isOpenConfirmation}
        onClose={onCloseConfirmation}
        onConfirm={onRevoke}
        isLoading={loadingRevokeAccessByCode}
      />

      <Flex direction={'column'} gap={5}>
        {listUserConsent?.map((consent, index) => {
          return (
            <Checkbox key={index} value={consent.code} onChange={onCheckboxChange}>
              <Flex key={index}
                align={'center'}
                gap={5}
                _hover={{ textDecoration: 'underline' }}
                pl={5}
              >
                <Icon as={FaHospital} boxSize={10} color={'primary.700'} />
                <Flex direction={'column'}>
                  <Text
                    fontSize={'xl'}
                    fontWeight={'bold'}
                  >
                    {consent.V1.display_name}
                  </Text>
                  <Text color={'gray.500'}>
                    {consent.V1.address}
                  </Text>
                </Flex>
              </Flex>
            </Checkbox>
          )
        })}
      </Flex>

      <SlideFade in={revokeConsent.length > 0}>
        <Button type="submit"
          colorScheme="primary"
          bg={"primary.700"}
          fontSize={'sm'}
          w={'full'}
          py={6}
          rounded={"xl"}
          onClick={onOpenConfirmation}
          isDisabled={revokeConsent.length === 0}
          isLoading={loadingRevokeAccessByCode}
        >
          {emrRevokeButton.label}
        </Button>
      </SlideFade>
    </Flex>
  )
}

export default function EMRRevokeList() {
  return (
    <ProviderActor canisterId={providerCanisterId}>
      <Flex flex={1}>
        <ConcentList />
      </Flex>
    </ProviderActor>
  )
}