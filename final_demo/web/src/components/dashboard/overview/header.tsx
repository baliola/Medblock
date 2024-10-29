"use client"

import { Button, Flex, FormControl, FormLabel, Input, Select } from "@chakra-ui/react";
import { useSearchParams } from "next/navigation";
import HospitalInfo from "./hospital-info";

export default function OverviewHeader() {
  const params = useSearchParams();

  const polyclinic = params.get("polyclinic") || "all";

  return (
    <Flex direction={"column"} gap={8}>
      <HospitalInfo />
      <Flex align={"end"} gap={5}>
        <FormControl w={"xs"}>
          <FormLabel color={"neutral.600"}>
            Polyclinic
          </FormLabel>
          <Select
            value={polyclinic}
            bg={"primary.200"}
            border={"none"}
            outline={"none"}
            rounded={"xl"}
            h={12}
            onChange={() => { }}
          >
            <option value={"all"}>All</option>
            <option value={"emergency"}>Emergency</option>
            <option value={"general"}>General</option>
            <option value={"pediatric"}>Pediatric</option>
            <option value={"other"}>Other</option>
          </Select>
        </FormControl>
        <FormControl w={"xs"}>
          <FormLabel color={"neutral.600"}>
            Date From
          </FormLabel>
          <FormControl>
            <Input bg={"primary.200"} border={"none"} outline={"none"} rounded={"xl"} h={12} type={"date"} />
          </FormControl>
        </FormControl>
        <FormControl w={"xs"}>
          <FormLabel color={"neutral.600"}>
            Date To
          </FormLabel>
          <FormControl>
            <Input bg={"primary.200"} border={"none"} outline={"none"} rounded={"xl"} h={12} type={"date"} />
          </FormControl>
        </FormControl>
        <Button colorScheme={"primary"} h={12} rounded={"xl"} px={8}>
          Filter
        </Button>
      </Flex>
    </Flex>
  )
}