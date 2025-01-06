"use client";

import { useParams, usePathname, useRouter, useSearchParams } from "next/navigation";
import { Fragment, useEffect } from "react";
import { Flex } from "@chakra-ui/react";

import {
  EmrListConsentRequest,
  EmrListConsentResponse,
} from "@/declarations/patient_registry/patient_registry.did";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { useEMRStore } from "@/store/patient-emr";

import EMRPatientEmpty from "@/components/dashboard/emr/empty";
import EMRHistory from "@/components/dashboard/emr/history";
import EMRProfile from "@/components/dashboard/emr/patient";
import EMRReport from "@/components/dashboard/emr/report";
import { patientCanisterId } from "@/config/canisters/patient.canister";

const EMRDataPatient = ({ id }: { id: string }) => {
  const params = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();

  const page = params.get("page") || 0;
  const limit = params.get("limit") || 10;

  const setUserHasEMR = useEMRStore((state) => state.setUserHasEMR);
  const setEmrs = useEMRStore((state) => state.setEMRS);
  const setLoading = useEMRStore((state) => state.setLoading);

  const { call, loading } = usePatientQuery({
    functionName: "emr_list_with_session",
    refetchOnMount: true,
    onSuccess(data) {
      console.log("PARAM GET SUCCESS");

      setUserHasEMR(true);

      // @ts-expect-error
      const datas: EmrListConsentResponse = data;
      setEmrs(datas);

      const length = datas?.emr.length;
      const latestData = datas?.emr[length - 1];

      const param = new URLSearchParams(params);
      param.set("record", latestData?.header.emr_id);
      param.set("provider", latestData?.header.provider_id);
      param.set("registry", latestData?.header.registry_id.toText());

      const href = `${pathname}?${param.toString()}`;
      router.push(href);

      return;
    },
    onError(error) {
      console.log("PARAM GET ERROR");
      console.log(error);
    },
    onLoading(loading) {
      setLoading(loading);
    },
  });

  useEffect(() => {
    const request: EmrListConsentRequest = {
      session_id: id,
      limit: Number(limit),
      page: Number(page),
    };

    // @ts-expect-error
    call([request]);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [id]);

  loading && <></>
  return <EMRPatientEmpty />;
};

export default function EMRPatient({ id }: { id: string }) {
  const userHasEMR = useEMRStore((state) => state.userHasEMR);
  const emrs = useEMRStore((state) => state.emrs);

  console.log("emrs", emrs);
  console.log("ID PARAMS emr patient", id);

  return (
    <Flex
      w="full"
      p={10}
      gap={7}
      minH={"100dvh"}
      overflowY={"auto"}
      direction={{ base: "column", lg: "row" }}
    >
      <PatientActor canisterId={patientCanisterId}>
        <EMRProfile id={id} />
        {userHasEMR ? (
          <Fragment>
            <EMRReport id={id} />
            <EMRHistory />
          </Fragment>
        ) : (
          <EMRDataPatient id={id} />
        )}
      </PatientActor>
    </Flex>
  );
}
