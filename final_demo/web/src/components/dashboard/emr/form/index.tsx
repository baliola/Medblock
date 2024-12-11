import { emrButton } from "@/constants/contents/dashboard/emr/button";
import { emrForm } from "@/constants/contents/dashboard/emr/form";
import { EMR } from "@/libs/yup/emr";
import { Flex, Divider, Button } from "@chakra-ui/react";
import { FormikErrors, useFormikContext } from "formik";
import EMRFormInfo from "./info";
import EMRFormRecipe from "./recipe";
import EMRFormReport from "./report";
import EMRFormVitalSign from "./vital-sign";
import { ReactNode } from "react";
import EMRFormHistory from "./history";
import EMRFormDischarge from "./discharge";

interface IEMRFormContentProps {
  loading: boolean
  header: ReactNode
  label: string
  drugAllergyValue: string
  foodAllergyValue: string
  dischargeValue: string
  setFieldValue: (field: string, value: any, shouldValidate?: boolean) => Promise<void | FormikErrors<{}>>
}

export default function EMRFormContent({ props }: { props: IEMRFormContentProps}) {
  const { errors, touched } = useFormikContext<EMR>();
  const { info } = emrForm;
  const { loading, header, label, drugAllergyValue, foodAllergyValue, dischargeValue, setFieldValue } = props
  
  return (
    <>
      <Flex w={'full'} bg={"primary.100"} p={5} rounded={"xl"} direction={'column'}>
        {header}
        <Divider py={3} borderColor={'primary.300'} />
        <Flex direction={'column'} gap={7}>
          <EMRFormInfo />
          <EMRFormVitalSign />
          <EMRFormHistory props={{ 
            drugAllergyValue, 
            foodAllergyValue,
            setFieldValue 
          }}/>
          <EMRFormReport />
        </Flex>
      </Flex>
      <Flex w={{ lg: "lg" }} direction={'column'} gap={5}>
        <EMRFormDischarge props={{ 
            value: dischargeValue,
            setFieldValue 
          }}/>
        <Button
          type="submit"
          colorScheme="primary"
          bg={'primary.700'}
          py={5}
          fontSize={'xs'}
          rounded={'lg'}
          isLoading={loading}
        >
          {label}
        </Button>
      </Flex>
    </>
  )
}