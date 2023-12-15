#include "cosave.h"

#include "lib.rs.h"

namespace cosave
{
	inline const auto CYCLE_RECORD   = _byteswap_ulong('CYCL');
	inline const auto FORMAT_VERSION = 1;

	void initializeCosaves()
	{
		const auto settings = user_settings();
		auto uniq           = settings->skse_identifier();
		rlog::info("Initializing cosave serialization.");
		auto* cosave = SKSE::GetSerializationInterface();
		cosave->SetUniqueID(uniq);
		cosave->SetSaveCallback(cosave::gameSavedHandler);
		cosave->SetRevertCallback(cosave::revertHandler);
		cosave->SetLoadCallback(cosave::gameLoadedHandler);
	}

	void gameSavedHandler(SKSE::SerializationInterface* cosave)
	{
		// The format is an ad-hoc bag of bytes that we interpret
		// as we wish. So we serialize to a bag of bytes on the Rust side.
		const uint32_t version    = serialize_version();
		rust::Vec<uint8_t> buffer = serialize_cycles();
		uint32_t bufsize          = static_cast<uint32_t>(buffer.size());
		rlog::debug("cycles serialized into a Vec<u8> of size={};"sv, bufsize);

		if (!cosave->OpenRecord(CYCLE_RECORD, version))
		{
			rlog::error("Unable to open record to write cosave data.");
			return;
		}

		cosave->WriteRecordData(bufsize);
		cosave->WriteRecordData(buffer.data(), bufsize);
	}

	void gameLoadedHandler(SKSE::SerializationInterface* cosave)
	{
		std::uint32_t type;
		std::uint32_t version;
		std::uint32_t size;

		while (cosave->GetNextRecordInfo(type, version, size))
		{
			if (type == CYCLE_RECORD)
			{
				rlog::info("reading cosave data version {}"sv, version);
				uint32_t bufSize;
				std::vector<uint8_t> buffer;
				cosave->ReadRecordData(bufSize);
				buffer.resize(bufSize);

				const auto read = cosave->ReadRecordData(buffer.data(), bufSize);
				buffer.resize(read);
				rlog::debug("read {} bytes from cosave; buffer len is {}"sv, read, buffer.size());
				cycle_loaded_from_cosave(buffer, version);
			}
			else { rlog::warn("Unknown record type in cosave; type={}", type); }
		}
	}

	void revertHandler(SKSE::SerializationInterface*) { clear_cache(); }
}
